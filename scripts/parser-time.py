import re
import csv

def parse_benchmark_output(input_text):
    # Regular expression to match the benchmark lines with time in nanoseconds
    pattern = r'(\w+)\s+time:\s*\[\d+\.\d+ ns (\d+\.\d+) ns \d+\.\d+ ns\]'
    
    # Store results
    results = []
    
    # Process each line
    for line in input_text.split('\n'):
        match = re.search(pattern, line)
        if match:
            opcode = match.group(1)
            median_time = float(match.group(2))  # Getting the median time
            results.append({'opcode': opcode, 'value': median_time})
    
    return results

def write_to_csv(results, output_file='avg-opcode-time.csv'):
    with open(output_file, 'w', newline='') as f:
        writer = csv.DictWriter(f, fieldnames=['opcode', 'value'])
        writer.writeheader()
        writer.writerows(results)

# Read the benchmark output from a file
with open('benchmark_output.txt', 'r') as f:
    benchmark_output = f.read()

# Parse and write to CSV
results = parse_benchmark_output(benchmark_output)
write_to_csv(results)