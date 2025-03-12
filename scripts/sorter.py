import csv
import sys

def sort_csv_by_key(input_file, key):
    all_rows = []

    # Read the CSV file
    with open(input_file, 'r', newline='') as f:
        reader = csv.DictReader(f)
        all_rows.extend(reader)

    # Sort all rows by the specified key
    sorted_rows = sorted(all_rows, key=lambda row: row[key])

    # Construct the output file name
    output_file = input_file.replace('.csv', '_sorted.csv')

    # Write the sorted rows to the output CSV file
    if sorted_rows:
        with open(output_file, 'w', newline='') as f:
            writer = csv.DictWriter(f, fieldnames=sorted_rows[0].keys())
            writer.writeheader()
            writer.writerows(sorted_rows)

if __name__ == "__main__":
    # Example usage: python script.py file.csv key
    if len(sys.argv) != 3:
        print("Usage: python sorter.py <file.csv> <key>")
    else:
        input_file = sys.argv[1]
        key = sys.argv[2]
        sort_csv_by_key(input_file, key)