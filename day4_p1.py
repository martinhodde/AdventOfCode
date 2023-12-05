from utils import read_file
inputs = read_file('input4.txt')

card_point_sum = 0
for card in inputs:
    # Determine winning numbers and numbers we own for each card
    winning_nums, owned_nums = map(lambda x : set(x.strip().split()), card.split(':')[1].split('|'))
    # Find how many winning numbers we have and compute card score accordingly
    num_winners_owned = len(owned_nums.intersection(winning_nums))
    card_point_sum += 2 ** (num_winners_owned - 1) if num_winners_owned > 0 else 0

print('Card point sum:', card_point_sum)
