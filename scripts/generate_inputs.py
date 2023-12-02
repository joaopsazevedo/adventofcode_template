#!/usr/bin/env python

import os
import os.path
import sys


def validate_year(year) -> int:
    try:
        year = int(year)
        if 2015 <= year <= 2023:
            return year
        else:
            raise ValueError("Year must be between 2015 and 2023.")
    except ValueError:
        print("Error: Please provide a valid year between 2015 and 2023.")
        sys.exit(1)


def create_inputs(validated_year: int):
    # Create the directory if it doesn't exist
    inputs_directory_path = os.path.join(
        os.path.curdir, "inputs", f"AoC{validated_year}"
    )
    if not os.path.exists(inputs_directory_path):
        os.makedirs(inputs_directory_path)
        print(f"Created directory: {inputs_directory_path}")

    # For each day, create an emptry input file if it doesn't exist
    for day in range(1, 26):
        input_filename = f"year{str(validated_year)}day{str(day).zfill(2)}.txt"
        # For days between 1 and 9, pad with 1 zero
        input_path = os.path.join(inputs_directory_path, input_filename)
        # Create the file if it doesn't exist
        if not os.path.exists(input_path):
            with open(input_path, "w") as _:
                print(f"Created input: {input_path}")


def main() -> None:
    if len(sys.argv) != 2:
        print("Error: Please provide a single argument for the year.")
        sys.exit(1)

    input_year = sys.argv[1]
    validated_year = validate_year(input_year)
    create_inputs(validated_year)


if __name__ == "__main__":
    main()
