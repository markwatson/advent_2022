import json
import functools

def parse_packets_list():
    packets = []
    for line in open("./data/day_13"):
        if line.strip() != "": 
            packets.append(json.loads(line.strip()))
    return packets

def parse_packets_pairs():
    packets = []
    packet =[]
    for line in open("./data/day_13"):
        if line.strip() == "": 
            packets.append(tuple(packet))
            packet = []
        else:
            packet.append(json.loads(line.strip()))
    packets.append(tuple(packet))
    return packets

def compare(left, right):
    if type(left) is list and type(right) is int:
        right = [right]
    elif type(left) is int and type(right) is list:
        left = [left]

    if type(left) is int and type(right) is int:
        if left < right:
            return True
        elif left > right:
            return False
        else:
            return None
    elif type(left) is list and type(left) is list:
        for item in zip(left, right):
            c = compare(item[0], item[1])
            if c is None:
                continue
            else:
                return c
    else:
        raise Exception("Invalid types: %s, %s" % (type(left), type(right)))
    
    if len(left) < len(right):
        return True
    elif len(left) > len(right):
        return False
    else:
        return None


# Part 1
packets = parse_packets_pairs()
num = 1
passed = 0
for packet in packets:
    left, right = packet
    
    result = compare(left, right)
    if result is None:
        raise Exception("Test %d failed" % num)
    if result:
        print("Test %d passed" % num)
        passed += num
    
    num += 1
    
print("Part 1: %d" % passed)

# Part 2
packets = parse_packets_list()
signal_1 = [[2]]
signal_2 = [[6]]
packets.append(signal_1)
packets.append(signal_2)

def sort_key(left, right):
    item = compare(left, right)
    if item == True:
        return -1
    elif item == False:
        return 1
    elif item == None:
        return 0

sorted_packets = sorted(packets, key=functools.cmp_to_key(sort_key))
index = 1
result = 1
for item in sorted_packets:
    print(item)
    if item == signal_1 or item == signal_2:
        result *= index
    index += 1
    
print("Part 2: %d" % result)