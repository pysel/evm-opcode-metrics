import csv

def filter_csv(file1, file2, output_file):
    # Read keys from the second file
    with open(file2, mode='r', newline='') as f2:
        reader = csv.reader(f2)
        keys_in_file2 = {row[0] for row in reader}

    # Filter the first file based on keys in the second file
    with open(file1, mode='r', newline='') as f1, open(output_file, mode='w', newline='') as out:
        reader = csv.reader(f1)
        writer = csv.writer(out)

        # Write header
        header = next(reader)
        writer.writerow(header)

        # Write only rows with keys present in file2
        for row in reader:
            if row[0] in keys_in_file2:
                writer.writerow(row)

# File to filter | File to match keys | Output file
filter_csv('avg-opcode-time-x86.csv', 'opcode_table.csv', 'avg-opcode-time-x86_sorted.csv')