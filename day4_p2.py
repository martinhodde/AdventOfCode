from utils import read_file
inputs = read_file('input4.txt')

# Perform an initial pass to build a map of card index numbers to their winning and owned numbers
card_num_map = {}
for card_idx, card in enumerate(inputs):
    # Determine winning numbers and numbers we own for each card
    winning_nums, owned_nums = map(lambda x : set(x.strip().split()), card.split(':')[1].split('|'))
    card_num_map[card_idx + 1] = (winning_nums, owned_nums)

memo = {}  # Memoize results
def compute_total_cards(card_nums):
    """
    Given a range of card numbers, compute the number of total scratch cards
    that will be won from them.
    """
    # Filter out card numbers that do not appear in the data
    card_subset = [(num, card_num_map[num]) for num in card_nums if num in card_num_map]
    total_scratch_cards = len(card_subset)

    for card_num, (winning, owned) in card_subset:
        # Find how many winning numbers we have for current card
        num_winners_owned = len(owned.intersection(winning))
        # Recursively compute total cards won from new cards if result has not already been cached
        new_cards = range(card_num + 1, card_num + num_winners_owned + 1)
        if new_cards not in memo:
            memo[new_cards] = compute_total_cards(new_cards)
        total_scratch_cards += memo[new_cards]

    return total_scratch_cards

num_scratch_cards = compute_total_cards(range(1, len(inputs) + 1))
print('Number of scratchcards:', num_scratch_cards)
