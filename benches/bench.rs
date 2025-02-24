// use criterion::{criterion_group, criterion_main, Criterion};
// use std::time::Instant;
// use std::collections::HashMap;
// use csv::Writer;
// use revm::{
//     interpreter::{Interpreter, OPCODE_INFO_JUMPTABLE},
//     primitives::CancunSpec,
//     Evm
// };
// use revm_interpreter::{
//     opcode::make_instruction_table,
//     Contract, DummyHost
// };
// use criterion_cycles_per_byte::CyclesPerByte;

// const ITERATIONS: usize = 10;

// pub fn criterion_benchmark(c: &mut Criterion<CyclesPerByte>) {
//     let mut group = c.benchmark_group("opcodes");

//     let evm = Evm::builder().build();
//     let mut interpreter = Interpreter::new(Contract::default(), 1_000_000, false);
//     let mut host = DummyHost::new(*evm.context.evm.env.clone());

//     let info_table = OPCODE_INFO_JUMPTABLE;
//     let instruction_table = make_instruction_table::<DummyHost, CancunSpec>();

//     let mut elapsed_map: HashMap<&str, Vec<u128>> = HashMap::new();
//     for (index, instruction) in instruction_table.iter().enumerate() {
//         if index == 88 { 
//             // this is the opcode for program counter instruction. It expects the instruction counter 
//             // to be offset by 1.
//             interpreter.instruction_pointer = unsafe { interpreter.instruction_pointer.offset(1) };
//         }

//         let op_code_info = info_table[index];
//         if let Some(op_code_info) = op_code_info {
//             let now = Instant::now();
//             group.bench_function(op_code_info.name(), |b| b.iter(|| instruction(&mut interpreter, &mut host)));
            
//             let elapsed = now.elapsed().as_nanos();
//             // Collect elapsed times in the vector for this opcode
//             elapsed_map.entry(op_code_info.name())
//                     .or_insert_with(Vec::new)
//                     .push(elapsed);
//         }
//     }
//     group.finish();
// }

// criterion_group!(
//     name = benches;
//     config = Criterion::default().with_measurement(CyclesPerByte);
//     targets = criterion_benchmark
// );
// criterion_main!(benches);