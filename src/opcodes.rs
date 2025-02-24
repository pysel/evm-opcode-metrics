use std::time::Instant;
use std::collections::HashMap;
use csv::Writer;
use revm::{
    interpreter::{Interpreter, OPCODE_INFO_JUMPTABLE},
    primitives::CancunSpec,
    Evm
};
use revm_interpreter::{
    opcode::make_instruction_table,
    Contract, DummyHost
};
use criterion::black_box;
use core::arch::x86_64::_rdtsc;


const ITERATIONS: usize = 10;

pub fn opcodes_time() {
    let evm = Evm::builder().build();
    let mut interpreter = Interpreter::new(Contract::default(), 1_000_000, false);
    let mut host = DummyHost::new(*evm.context.evm.env.clone());

    let info_table = OPCODE_INFO_JUMPTABLE;
    let instruction_table = make_instruction_table::<DummyHost, CancunSpec>();

    let mut elapsed_map: HashMap<&str, Vec<u128>> = HashMap::new();
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
                // Collect elapsed times in the vector for this opcode
                elapsed_map.entry(op_code_info.name())
                        .or_insert_with(Vec::new)
                        .push(elapsed);
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
                black_box(instruction(&mut interpreter, &mut host));
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
