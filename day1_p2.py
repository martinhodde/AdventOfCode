from utils import read_file
inputs = read_file('input1.txt')

digit_words = {
    # Overlapping digit word edge cases appearing in inputs
    'twone' : '21',
    'oneight' : '18',
    'eightwo' : '82',
    # Regular old digits
    'one' : '1',
    'two' : '2',
    'three' : '3',
    'four' : '4',
    'five' : '5',
    'six' : '6',
    'seven' : '7',
    'eight' : '8',
    'nine' : '9'
}

calibration_sum = 0
for line in inputs:
    # Perform a preliminary pass over each input line to replace
    # digit words with their corresonding digits
    for digit_word, digit in digit_words.items():
        if digit_word in line:
            line = line.replace(digit_word, digit)

    # Proceed as in the previous part
    first_digit, last_digit = None, None
    for character in line:
        if character.isdigit():
            if first_digit is None:
                first_digit = character
            last_digit = character

    calibration_value = int(first_digit + last_digit)
    calibration_sum += calibration_value

print('Calibration sum:', calibration_sum)
