def decode_pos(pos):
    pad = [[1,2,3], [4,5,6], [7,8,9]]
    return pad[pos[1]][pos[0]]

def decode_position(pos):
    pad = [
        [ 0,  0,  1,  0,  0 ],
        [ 0,  2,  3,  4,  0 ],
        [ 5,  6,  7,  8,  9 ],
        [ 0, 'A','B','C', 0 ],
        [ 0,  0, 'D', 0,  0 ],
    ]
    return pad[pos[1]][pos[0]]

def next_location(current_location, instruction):
    valid_pad = [
        [0, 0, 1, 0, 0],
        [0, 1, 1, 1, 0],
        [1, 1, 1, 1, 1],
        [0, 1, 1, 1, 0],
        [0, 0, 1, 0, 0],
    ]
    row_len = len(valid_pad[0]) - 1
    x = current_location[0]
    y = current_location[1]
    
    dir_index = {'U': (0, -1), 'D': (0, 1), 'L': (-1, 0), 'R': (1, 0)}
    delta = dir_index[instruction]
    
    x = x + delta[0]
    y = y + delta[1]
    
    if x < 0:
        x = 0
    if x > row_len:
        x = row_len
    if y < 0:
        y = 0
    if y > row_len:
        y = row_len
    
    new_pos = (x, y)
    if valid_pad[y][x] == 0:
        new_pos = current_location
    return new_pos

def move_sequence(start_from, sequence):
    pos = start_from
    for s in sequence:
        pos = next_location(pos, s)
    return pos

def main():
    #codes = ['ULL', 'RRDDD', 'LURDL', 'UUUUD']
    codes = open('input.txt')
    pos = (0, 2)
    for l in codes:
        pos = move_sequence(pos, l.strip())
        #print(decode_pos(pos))
        print(decode_position(pos))

main()

