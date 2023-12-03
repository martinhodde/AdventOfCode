from utils import read_file
inputs = read_file('input1.txt')

calibration_sum = 0
for line in inputs:
    first_digit, last_digit = None, None
    for character in line:
        if character.isdigit():
            if first_digit is None:
                # Only set first digit if we have not yet encountered any digits in the current input line
                first_digit = character
            last_digit = character  # Update last digit unconditionally

    calibration_value = int(first_digit + last_digit)
    calibration_sum += calibration_value

print('Calibration sum:', calibration_sum)
