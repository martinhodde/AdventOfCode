from utils import read_file
schematic = read_file('input3.txt')

# On the first pass through the schematic, we find the bounding boxes for all part numbers
part_num_bboxes = {}
curr_part_num = ''
for i, line in enumerate(schematic):
    for j, character in enumerate(line):
        
        if character.isdigit():
            if not curr_part_num:
                # In this case, we have encountered the start of a number,
                # so we note the top-left coordinates of the bounding box
                top, left = i - 1, j - 1
            curr_part_num += character

        elif curr_part_num:
            # Here, we have reached the end of the number so we close the bounding box
            bottom, right = i + 1, j
            part_num_bboxes[(top, left, bottom, right)] = int(curr_part_num)
            curr_part_num = ''  # Reset current part number

# On the second pass, we check all gear symbols for intersections with two bounding boxes
gear_ratio_sum = 0
for i, line in enumerate(schematic):
    for j, character in enumerate(line.strip()):
        if character == '*':
            # Check gear location against all part number bounding boxes
            part_num_count, gear_ratio = 0, 1
            for (top, left, bottom, right), part_num in part_num_bboxes.items():
                if i in range(top, bottom + 1) and j in range(left, right + 1):
                    gear_ratio *= part_num
                    part_num_count += 1

            # Only add gear ratio if the gear is adjacent to exactly two part numbers
            if part_num_count == 2:
                gear_ratio_sum += gear_ratio

print('Gear ratio sum:', gear_ratio_sum)
