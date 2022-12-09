import re, json

re_dir = re.compile(r"^\$ cd (.+)$")
re_file = re.compile(r"^(\d+) (.+)$")

def main():    
    items = {}
    current_dir = []
    with open('./data/day_7', 'r') as f:
        for L in f:
            # Handle CD command
            dir_m = re_dir.match(L)
            if dir_m:
                dir = dir_m.group(1).strip()
                if dir == '..':
                    current_dir = current_dir[:-1]
                elif dir == '/':
                    current_dir = []
                else:
                    current_dir.append(dir)
            
            if tuple(current_dir) not in items:
                items[tuple(current_dir)] = {
                    'size': 0,
                    'is_dir': True,
                }
            
            # Handle listing
            file_m = re_file.match(L)
            if file_m:
                size = int(file_m.group(1))
                file = file_m.group(2).strip()
                items[tuple(current_dir + [file])] = {
                    'size': size,
                    'is_dir': False,
                }
                
    # Find dir sizes:
    print("parsed_items: " + str(items))
    for k, v in items.items():
        if not v['is_dir'] and len(k) > 0:
            for x in range(1, len(k) + 1):
                items[tuple(k[:-x])]['size'] += v['size']
            
    step_1_size = 0    
    for k, v in items.items():
        if v['is_dir'] and v['size'] <= 100000:
            step_1_size += v['size']
            
    print("Step 1: {}".format(step_1_size))
    
    disk_size = 70_000_000
    required_free = 30_000_000
    used = items[tuple()]['size']
    
    free_space = disk_size - used
    to_remove = required_free - free_space
    
    smallest = 1_000_000_000_000
    for k, v in items.items():
        if v['is_dir'] and v['size'] >= to_remove and v['size'] < smallest:
            smallest = v['size']
    
    print("Step 2: {}".format(smallest))

if __name__ == "__main__":
    main()