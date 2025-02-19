use std::time::Instant;
use std::collections::HashMap;
use csv::Writer;
use revm::{
    interpreter::{Interpreter, OpCode, OPCODE_INFO_JUMPTABLE},
    primitives::{
        address, bytes, Bytecode, Bytes, CancunSpec, HashMap as RevmHashMap, Spec,
        TxKind,
    },
    Evm, Frame, FrameOrResult, InMemoryDB,
};
use revm_interpreter::{
    opcode::{instruction, make_instruction_table, OpCodeInfo},
    Contract, DummyHost,
};

use std::hint::black_box;


const ITERATIONS: usize = 100;

fn main() {
    let evm = Evm::builder().build();
    let mut interpreter = Interpreter::new(Contract::default(), 1_000_000, false);
    let mut host = DummyHost::new(*evm.context.evm.env.clone());

    let info_table = OPCODE_INFO_JUMPTABLE;
    let instruction_table = make_instruction_table::<DummyHost, CancunSpec>();

    let mut elapsed_map: HashMap<&str, u128> = HashMap::new();
    for _ in 0..ITERATIONS {
        for (index, instruction) in instruction_table.iter().enumerate() {
            if index == 88 { 
                // this is the opcode for program counter instruction. It expects the instruction counter 
                // to be offset by 1.
                interpreter.instruction_pointer = unsafe { interpreter.instruction_pointer.offset(1) };
            }

            let op_code_info = info_table[index];
            if let Some(op_code_info) = op_code_info {
                let now = Instant::now();
                black_box(instruction(&mut interpreter, &mut host));
                let elapsed = now.elapsed().as_nanos();

                // Add elapsed time for this run to the total time for this opcode
                *elapsed_map.entry(op_code_info.name()).or_insert(0) += elapsed;
            }
        }
    }

    let mut elapsed_avg_map: HashMap<&str, u128> = HashMap::new();
    for (opcode, elapsed) in elapsed_map {
        elapsed_avg_map.insert(opcode, elapsed / ITERATIONS as u128);
    }

    // Collect and sort the keys
    let mut sorted_opcodes: Vec<&str> = elapsed_avg_map.keys().cloned().collect();
    sorted_opcodes.sort();

    let mut wtr = Writer::from_path("avg-opcode-time.csv").unwrap();
    wtr.write_record(&["opcode", "avg_time"]).unwrap();
    for opcode in sorted_opcodes {
        if let Some(elapsed) = elapsed_avg_map.get(opcode) {
            wtr.write_record(&[opcode, &elapsed.to_string()]).unwrap();
        }
    }
    wtr.flush().unwrap();
}

