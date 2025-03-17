use criterion::{criterion_group, criterion_main, Criterion};
use revm_primitives::{bitvec::{order::Lsb0, vec::BitVec}, eof::TypesSection, Address, Bytecode, JumpTable, LegacyAnalyzedBytecode, U256};
use revm::primitives::bytes::Bytes;
use std::{sync::Arc, time::Instant};
use std::collections::HashMap;
use revm::primitives::Eof;

use revm::{
    interpreter::{Interpreter, OPCODE_INFO_JUMPTABLE},
    primitives::CancunSpec,
    Evm
};
use revm_interpreter::{
    opcode::make_instruction_table,
    Contract, DummyHost
};

// use criterion_cycles_per_byte::CyclesPerByte;

const ITERATIONS: usize = 10;

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

pub fn criterion_benchmark(c: &mut Criterion) {
    let evm = Evm::builder().build();
    let mut host = DummyHost::new(*evm.context.evm.env.clone());

    let info_table = OPCODE_INFO_JUMPTABLE;
    let instruction_table = make_instruction_table::<DummyHost, CancunSpec>();

    let mut elapsed_map: HashMap<&str, Vec<u128>> = HashMap::new();
    for (index, instruction) in instruction_table.iter().enumerate() {
        let mut interpreter_eof = get_eof_interpreter();

        let op_code_info = info_table[index];
        if let Some(op_code_info) = op_code_info {
            // let mut result: revm_interpreter::InstructionResult = revm_interpreter::InstructionResult::Stop;

            if EOF_OPCODES.contains(&op_code_info.name()) {
                for _ in 0..7 {
                    let _ = interpreter_eof.stack.push(U256::from(0));
                }
                c.bench_function(op_code_info.name(), |b| 
                    b.iter_batched(
                    || {
                        let mut interpreter = get_eof_interpreter();

                        // Create a mutable reference to interpreter for stack setup
                        for _ in 0..50 {
                            let _ = interpreter.stack.push(U256::from(0));
                        }

                        if DIRTY_STACK_OPCODES.contains(&op_code_info.name()) {
                            for _ in 0..7 {
                                let _ = interpreter.stack.push(U256::from(1));
                            }
                        }

                        interpreter
                    },
                    |mut interpreter| {
                        instruction(&mut interpreter, &mut host);
                        // result = interpreter.instruction_result;
                    },
                    criterion::BatchSize::SmallInput
                    )
                );
                // println!("{}: {:?}", op_code_info.name(), result);
                continue;
            } 

            let now = Instant::now();
            c.bench_function(op_code_info.name(), |b| {
                b.iter_batched(
                    || {
                        let (mut interpreter, bytecode_ptr) = get_legacy_analyzed_interpreter();

                        if index == 88 { 
                            // this is the opcode for program counter instruction. It expects the instruction counter 
                            // to be offset by 1.
                            interpreter.instruction_pointer = bytecode_ptr.wrapping_add(1);
                        }
                        // Create a mutable reference to interpreter for stack setup
                        for _ in 0..50 {
                            let _ = interpreter.stack.push(U256::from(0));
                        }

                        if DIRTY_STACK_OPCODES.contains(&op_code_info.name()) {
                            for _ in 0..7 {
                                let _ = interpreter.stack.push(U256::from(1));
                            }
                        }

                        interpreter
                    },
                    |mut interpreter| {
                        instruction(&mut interpreter, &mut host);
                        // result = interpreter.instruction_result;
                    },
                    criterion::BatchSize::SmallInput
                );  
            });

            // println!("{}: {:?}", op_code_info.name(), result);
            let elapsed = now.elapsed().as_nanos();

            // println!("{}: {:?}", op_code_info.name(), interpreter.instruction_result);
            // Collect elapsed times in the vector for this opcode
            elapsed_map.entry(op_code_info.name())
                    .or_insert_with(Vec::new)
                    .push(elapsed);

        }
    }
}


fn get_legacy_analyzed_interpreter() -> (Interpreter, *const u8) {
    let mut bit_vec: BitVec<u8, Lsb0> = BitVec::new();
    bit_vec.push(true);
    bit_vec.push(true);
    let jump_table = JumpTable::from_slice(bit_vec.as_raw_slice());

    let bytecode = LegacyAnalyzedBytecode::new(revm_primitives::Bytes(Bytes::from_static(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10])), 1, jump_table);
    let bytecode_ptr = bytecode.bytecode().as_ptr();
    let contract = Contract::new(revm_primitives::Bytes::default(), Bytecode::LegacyAnalyzed(bytecode), None, Address::ZERO, None, Address::ZERO, U256::ZERO);

    // return;
    
    let interpreter = Interpreter::new(contract, 1_000_000_000_000, false);
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


criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);