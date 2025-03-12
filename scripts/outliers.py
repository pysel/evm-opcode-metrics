import pandas as pd

def filter_opcodes(input_file, output_file, value_key='value', threshold=0.5):
    # Read the CSV file
    df = pd.read_csv(input_file)
    
    print(df.head())

    # Calculate the mean of the specified value column
    mean_value = df[value_key].mean()

    print(f"Mean value: {mean_value}")
    # Calculate the lower and upper bounds
    lower_bound = mean_value * (1 - threshold)
    upper_bound = mean_value * (1 + threshold)

    # Create a copy of the DataFrame to avoid mutating the original
    df_copy = df.copy()

    # Filter the DataFrame for values outside the bounds
    filtered_df = df_copy[(df_copy[value_key] < lower_bound) | (df_copy[value_key] > upper_bound)]

    # Write the filtered DataFrame to a new CSV file
    filtered_df.to_csv(output_file, index=False)

# Example usage
directory = '../outliers/'
input_csvs = ['avg-opcode-time-x86.csv', 'avg-opcode-time-arm.csv', 'avg-opcode-cycles-arm.csv', 'avg-opcode-cycles-x86.csv']
thresholds = [0.2, 0.2, 0.5, 0.5]
for input_csv, threshold in zip(input_csvs, thresholds):
    output_csv = directory + input_csv
    filter_opcodes("../" + input_csv, output_csv, 'value', threshold)