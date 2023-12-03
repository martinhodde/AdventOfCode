import math

from utils import read_file
inputs = read_file('input2.txt')

power_sum = 0
for game_idx, line in enumerate(inputs):
    # Remove commas and semicolons, then separate by whitespace
    line = line.replace(',', '').replace(';', '')
    game = line.split()
    
    min_cubes_req = {'red' : 1, 'green' : 1, 'blue' : 1}
    for str_idx, game_str in enumerate(game):
        if game_str in ['red', 'green', 'blue']:
            num_cubes = int(game[str_idx - 1])
            # Set min required cube number to the max for each color
            min_cubes_req[game_str] = max(num_cubes, min_cubes_req[game_str])

    power_sum += math.prod(min_cubes_req.values())  # Compute power and add to total

print('Power sum:', power_sum)
