from utils import read_file
inputs = read_file('input2.txt')

# Maximum configured cube number for each color
max_cubes = {'red' : 12, 'green' : 13, 'blue' : 14}

game_id_sum = 0
for game_idx, line in enumerate(inputs):
    # Remove commas and semicolons, then separate by whitespace
    line = line.replace(',', '').replace(';', '')
    game = line.split()
    
    game_id_sum += game_idx + 1  # Preemptively add id to sum
    for str_idx, game_str in enumerate(game):
        # Check if each number of cubes in input line is possible
        if game_str in ['red', 'green', 'blue']:
            num_cubes = int(game[str_idx - 1])
            if num_cubes > max_cubes[game_str]:
                # Subtract id from sum if game is impossible
                game_id_sum -= game_idx + 1
                break

print('Game ID sum:', game_id_sum)
