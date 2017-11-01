def get_new_direction(current_direction, turn_to):
    directions = ['N', 'E', 'S', 'W']
    dir_index = {'L': -1, 'R': 1}
    new_direction = (directions.index(current_direction) + dir_index[turn_to]) % len(directions)
    return directions[new_direction]

def get_new_position(current_position, direction, distance):
    directions = ['N', 'E', 'S', 'W']
    deltas = [(0, 1), (1, 0), (0, -1), (-1, 0)]
    
    xd = deltas[directions.index(direction)]
    d = (xd[0] * distance, xd[1] * distance)
    
    return (current_position[0] + d[0], current_position[1] + d[1])

def get_position(current_position, current_direction, instruction):
    turn_to = instruction[0]
    distance = int(instruction[1:])
    
    new_direction = get_new_direction(current_direction, turn_to)
    new_position = get_new_position(current_position, new_direction, distance)
    
    return {'dir': new_direction, 'pos': new_position}

def distance_from_origin(pos):
    return abs(pos[0]) + abs(pos[1])

def west(line):
    point1 = line[0]
    point2 = line[1]
    if point1[0] > point2[0]:
        return point2[0]
    return point1[0]

def east(line):
    point1 = line[0]
    point2 = line[1]
    if point1[0] < point2[0]:
        return point2[0]
    return point1[0]

def north(line):
    point1 = line[0]
    point2 = line[1]
    if point1[1] < point2[1]:
        return point2[1]
    return point1[1]

def south(line):
    point1 = line[0]
    point2 = line[1]
    if point1[1] > point2[1]:
        return point2[1]
    return point1[1]

def get_lines_intersect_point(line1, line2):
    
    if north(line1) < south(line2):
        return None
    if south(line1) > north(line2):
        return None
    if east(line1) < west(line2):
        return None
    if west(line1) > east(line2):
        return None
    
    # line1 is horizontal
    if line1[0][0] == line1[1][0]:
        return (line1[0][0], line2[0][1])
    
    return (line2[0][0], line1[0][1])


def find_intersections(point_list):
    for p in range(len(point_list) - 3):
        latest_line = (point_list[p+2], point_list[p+3])
        for x in range(p+1):
            line = (point_list[x], point_list[x+1])
            ip = get_lines_intersect_point(line, latest_line)
            #print('Comparing ' + str(line) + ' and ' + str(latest_line))
            if ip is not None:
                print('Comparing ' + str(line) + ' and ' + str(latest_line))
                print('Intersect! At ' + str(ip) + ', distance = ' + str(distance_from_origin(ip)))
                print('Lines ' + str(x) + ' and ' + str(p))
                return


def main():
    current_direction = 'N'
    current_position = (0, 0)
    #instructions = 'L2, L3, L3, L4, R1, R2, L3, R3, R3, L1, L3, R2, R3, L3, R4, R3, R3, L1, L4, R4, L2, R5, R1, L5, R1, R3, L5, R2, L2, R2, R1, L1, L3, L3, R4, R5, R4, L1, L189, L2, R2, L5, R5, R45, L3, R4, R77, L1, R1, R194, R2, L5, L3, L2, L1, R5, L3, L3, L5, L5, L5, R2, L1, L2, L3, R2, R5, R4, L2, R3, R5, L2, L2, R3, L3, L2, L1, L3, R5, R4, R3, R2, L1, R2, L5, R4, L5, L4, R4, L2, R5, L3, L2, R4, L1, L2, R2, R3, L2, L5, R1, R1, R3, R4, R1, R2, R4, R5, L3, L5, L3, L3, R5, R4, R1, L3, R1, L3, R3, R3, R3, L1, R3, R4, L5, L3, L1, L5, L4, R4, R1, L4, R3, R3, R5, R4, R3, R3, L1, L2, R1, L4, L4, L3, L4, L3, L5, R2, R4, L2'
    instructions = 'L2, L3, L3, L4, R1, R2, L3, R3, R3, L1, L3, R2, R3, L3, R4, R3, R3, L1, L4, R4, L2, R5, R1, L5, R1, R3, L5, R2, L2, R2, R1, L1, L3, L3, R4, R5, R4, L1, L189, L2, R2, L5, R5, R45, L3, R4, R77, L1, R1, R194, R2, L5, L3, L2, L1, R5, L3, L3, L5, L5, L5, R2, L1, L2, L3, R2, R5, R4, L2, R3, R5, L2, L2, R3, L3, L2, L1, L3, R5, R4, R3, R2, L1, R2, L5, R4, L5, L4, R4, L2, R5, L3, L2, R4, L1, L2, R2, R3, L2, L5, R1, R1, R3, R4, R1, R2, R4, R5, L3, L5, L3, L3, R5, R4, R1, L3, R1, L3, R3, R3, R3, L1, R3, R4, L5, L3, L1, L5, L4, R4, R1, L4, R3, R3, R5, R4, R3, R3, L1, L2, R1, L4, L4, L3, L4, L3, L5, R2, R4, L2'

    positions = []
    positions.append(current_position)
    
    for instruction in instructions.split(', '):
        #print('At ' + str(current_position) + ', heading ' + current_direction + ', going to ' + instruction, end='')
        d = get_position(current_position, current_direction, instruction)
        current_position = d['pos']
        current_direction = d['dir']
        # print(' -> ' + str(current_position))
        positions.append(current_position)
    
    find_intersections(positions)
    print('Final position: ' + str(current_position))
    print('Distance: ' + str(distance_from_origin(current_position)))


main()