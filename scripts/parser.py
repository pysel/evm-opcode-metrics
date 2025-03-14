import re
import csv

def parse_benchmark_output(input_text):
    # Regular expression to match the benchmark lines
    pattern = r'opcodes/(\w+)\s+time:\s*\[\d+\.\d+ cycles (\d+\.\d+) cycles \d+\.\d+ cycles\]'
    
    # Store results
    results = []
    
    # Process each line
    for line in input_text.split('\n'):
        match = re.search(pattern, line)
        if match:
            opcode = match.group(1)
            median_cycles = float(match.group(2))
            results.append({'opcode': opcode, 'value': median_cycles})
    
    return results

def write_to_csv(results, output_file='avg-opcode-time-x86.csv'):
    with open(output_file, 'w', newline='') as f:
        writer = csv.DictWriter(f, fieldnames=['opcode', 'value'])
        writer.writeheader()
        writer.writerows(results)

# Read the benchmark output from a file
with open('../benchmark_output.txt', 'r') as f:
    benchmark_output = f.read()

# Parse and write to CSV
results = parse_benchmark_output(benchmark_output)
write_to_csv(results)