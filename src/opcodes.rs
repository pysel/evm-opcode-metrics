use std::{sync::Arc, time::Instant};
use std::collections::HashMap;
use csv::Writer;
use revm::primitives::bitvec::order::Lsb0;
use revm::primitives::bitvec::vec::BitVec;
use revm::primitives::bytes::Bytes;
use revm::primitives::{Eof, JumpTable, LegacyAnalyzedBytecode};
use revm::{
    interpreter::{Interpreter, OPCODE_INFO_JUMPTABLE},
    primitives::{Bytecode, CancunSpec, U256},
    Evm
};
use revm_interpreter::{
    opcode::make_instruction_table,
    Contract, DummyHost
};
use revm_primitives::eof::TypesSection;
use revm_primitives::Address;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use core::arch::x86_64::_rdtsc;


const ITERATIONS: usize = 1;

const DIRTY_STACK_OPCODES: [&str; 11] = [
    "LOG0", "LOG1", "LOG2", "LOG3", "LOG4", "CALL", "CALLCODE", "DELEGATECALL", "STATICCALL", "CREATE2", "CREATE"
];

const EOF_OPCODES: [&str; 18] = [
    "DATALOAD",
    "DATALOADN",
    "DATASIZE",
    "DATACOPY",
    "RJUMP",
    "RJUMPI",
    "RJUMPV",
    "CALLF",
    "RETF",
    "JUMPF",
    "DUPN",
    "SWAPN",
    "EXCHANGE",
    "EOFCREATE",
    "RETURNDATALOAD",
    "EXTCALL",
    "EXTDELEGATECALL",
    "EXTSTATICCALL",
];

pub fn opcodes_time() {
    let evm = Evm::builder().build();
    // let mut contract = Contract::default();
    // let eof = Arc::new(Eof::default());

    
    let mut host = DummyHost::new(*evm.context.evm.env.clone());

    let info_table = OPCODE_INFO_JUMPTABLE;
    let instruction_table = make_instruction_table::<DummyHost, CancunSpec>();

    let mut elapsed_map: HashMap<&str, Vec<u128>> = HashMap::new();
    for _ in 0..ITERATIONS {
        for (index, instruction) in instruction_table.iter().enumerate() {
            if false && index >= instruction_table.len() / 2 {
                break;
            } else if !false && index < instruction_table.len() / 2 {
                continue;
            }
            let mut interpreter_eof = get_eof_interpreter();

            let (mut interpreter, bytecode_ptr) = get_legacy_analyzed_interpreter();
            for _ in 0..50 {
                let _ = interpreter.stack.push(U256::from(0));
                let _ = interpreter_eof.stack.push(U256::from(0));
            }

            if index == 88 { 
                // this is the opcode for program counter instruction. It expects the instruction counter 
                // to be offset by 1.
                interpreter.instruction_pointer = bytecode_ptr.wrapping_add(1);

                println!("DATA: {:?}", unsafe { core::slice::from_raw_parts(interpreter.instruction_pointer, 1) });
            }

            let op_code_info = info_table[index];
            if let Some(op_code_info) = op_code_info {
                if DIRTY_STACK_OPCODES.contains(&op_code_info.name()) {
                    for _ in 0..7 {
                        let _ = interpreter.stack.push(U256::from(1));
                    }
                }

                if EOF_OPCODES.contains(&op_code_info.name()) {
                    for _ in 0..7 {
                        let _ = interpreter_eof.stack.push(U256::from(0));
                    }
                    instruction(&mut interpreter_eof, &mut host);
                    println!("{}: {:?}", op_code_info.name(), interpreter_eof.instruction_result);
                    continue;
                } else {
                    interpreter.is_eof = false;
                }

                let now = Instant::now();
                instruction(&mut interpreter, &mut host);
                
                let elapsed = now.elapsed().as_nanos();

                println!("{}: {:?}", op_code_info.name(), interpreter.instruction_result);
                // Collect elapsed times in the vector for this opcode
                elapsed_map.entry(op_code_info.name())
                        .or_insert_with(Vec::new)
                        .push(elapsed);

            }

            if index == 88 { 
                // this is the opcode for program counter instruction. It expects the instruction counter 
                // to be offset by 1.
                interpreter.instruction_pointer = bytecode_ptr;
            }
        }
    }

    let mut elapsed_avg_map: HashMap<&str, u128> = HashMap::new();
    let mut elapsed_median_map: HashMap<&str, u128> = HashMap::new();

    for (opcode, times) in &elapsed_map {
        // Compute average
        let sum: u128 = times.iter().sum();
        let avg = sum / ITERATIONS as u128;
        elapsed_avg_map.insert(*opcode, avg);

        // Compute median
        let mut sorted_times = times.clone();
        sorted_times.sort();
        let median = if ITERATIONS % 2 == 0 {
            (sorted_times[ITERATIONS / 2 - 1] + sorted_times[ITERATIONS / 2]) / 2
        } else {
            sorted_times[ITERATIONS / 2]
        };
        elapsed_median_map.insert(*opcode, median);
    }

    // Collect and sort the keys
    let mut sorted_opcodes: Vec<&str> = elapsed_avg_map.keys().cloned().collect();
    sorted_opcodes.sort();

    let mut wtr = Writer::from_path("avg-opcode-time.csv").unwrap();
    wtr.write_record(&["opcode", "avg_time", "median_time"]).unwrap();
    for opcode in sorted_opcodes {
        if let (Some(avg), Some(median)) = (
            elapsed_avg_map.get(opcode),
            elapsed_median_map.get(opcode),
        ) {
            wtr.write_record(&[
                opcode,
                &avg.to_string(),
                &median.to_string(),
            ]).unwrap();
        }
    }
    wtr.flush().unwrap();
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn opcodes_cycles() {
    let evm = Evm::builder().build();
    let mut interpreter = Interpreter::new(Contract::default(), 1_000_000, false);
    let mut host = DummyHost::new(*evm.context.evm.env.clone());

    let info_table = OPCODE_INFO_JUMPTABLE;
    let instruction_table = make_instruction_table::<DummyHost, CancunSpec>();

    let mut elapsed_map: HashMap<&str, Vec<u64>> = HashMap::new();
    for _ in 0..ITERATIONS {
        for (index, instruction) in instruction_table.iter().enumerate() {
            if index == 88 { 
                // this is the opcode for program counter instruction. It expects the instruction counter 
                // to be offset by 1.
                interpreter.instruction_pointer = unsafe { interpreter.instruction_pointer.offset(1) };
            }

            let op_code_info = info_table[index];
            if let Some(op_code_info) = op_code_info {
                let start = unsafe { _rdtsc() };
                instruction(&mut interpreter, &mut host);
                let end = unsafe { _rdtsc() };
                
                let elapsed = end - start;
                // Collect elapsed times in the vector for this opcode
                elapsed_map.entry(op_code_info.name())
                        .or_insert_with(Vec::new)
                        .push(elapsed);
            }
        }
    }

    let mut elapsed_avg_map: HashMap<&str, u64> = HashMap::new();
    let mut elapsed_median_map: HashMap<&str, u64> = HashMap::new();

    for (opcode, times) in &elapsed_map {
        // Compute average
        let sum: u64 = times.iter().sum();
        let avg: u64 = sum / ITERATIONS as u64;
        elapsed_avg_map.insert(*opcode, avg);

        // Compute median
        let mut sorted_times = times.clone();
        sorted_times.sort();
        let median = if ITERATIONS % 2 == 0 {
            (sorted_times[ITERATIONS / 2 - 1] + sorted_times[ITERATIONS / 2]) / 2
        } else {
            sorted_times[ITERATIONS / 2]
        };
        elapsed_median_map.insert(*opcode, median);
    }

    // Collect and sort the keys
    let mut sorted_opcodes: Vec<&str> = elapsed_avg_map.keys().cloned().collect();
    sorted_opcodes.sort();

    let mut wtr = Writer::from_path("avg-opcode-cycles.csv").unwrap();
    wtr.write_record(&["opcode", "avg_cycles", "median_cycles"]).unwrap();
    for opcode in sorted_opcodes {
        if let (Some(avg), Some(median)) = (
            elapsed_avg_map.get(opcode),
            elapsed_median_map.get(opcode),
        ) {
            wtr.write_record(&[
                opcode,
                &avg.to_string(),
                &median.to_string(),
            ]).unwrap();
        }
    }
    wtr.flush().unwrap();
}


// fn get_bytecode() -> Bytecode {
//     let buf: Bytes = Bytes::from_static(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    
//     // Create the bytecode using the new_analyzed method
//     let bytecode = Bytecode::LegacyRaw(buf);
//     bytecode
// }

fn get_legacy_analyzed_interpreter() -> (Interpreter, *const u8) {
    let mut bit_vec: BitVec<u8, Lsb0> = BitVec::new();
    bit_vec.push(true);
    bit_vec.push(true);
    let jump_table = JumpTable::from_slice(bit_vec.as_raw_slice());

    let bytecode = LegacyAnalyzedBytecode::new(revm_primitives::Bytes(Bytes::from_static(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10])), 1, jump_table);
    let bytecode_ptr = bytecode.bytecode().as_ptr();
    let contract = Contract::new(revm_primitives::Bytes::default(), Bytecode::LegacyAnalyzed(bytecode), None, Address::ZERO, None, Address::ZERO, U256::ZERO);

    // return;
    
    let interpreter = Interpreter::new(contract, 1_000_000_000, false);
    (interpreter, bytecode_ptr)
}

fn get_eof_interpreter() -> Interpreter {
    let mut bytecode = Eof::default();
    let eof = Eof::encode_slow(&bytecode);

    bytecode.body.types_section = Vec::with_capacity(1 << 10);
    bytecode.body.types_section.resize(34000, TypesSection::default());
    bytecode.body.code_section = Vec::with_capacity(1 << 10);
    bytecode.body.code_section.resize(34000, revm_primitives::Bytes(Bytes::from_static(&[0x00])));
    bytecode.body.container_section = Vec::with_capacity(1 << 10);
    bytecode.body.container_section.resize(34000, eof);


    let contract = Contract::new(revm_primitives::Bytes::default(), Bytecode::Eof(Arc::new(bytecode)), None, Address::ZERO, None, Address::ZERO, U256::ZERO);

    let mut interpreter = Interpreter::new(contract, 1_000_000_000, false);
    interpreter.function_stack.push(0, 0);

    interpreter
}
