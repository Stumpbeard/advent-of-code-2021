def part_1(lines):
    freq_map = {}
    length = 0
    for l in lines:
        line = l.strip()
        length = len(line)
        for i, c in enumerate(line):
            if i not in freq_map:
                freq_map[i] = 0
            if int(c) == 1:
                freq_map[i] += 1
            else:
                freq_map[i] -= 1

    gamma = ""
    beta = ""
    for i in range(length):
        gamma += "1" if freq_map[i] > 0 else "0"
        beta += "1" if freq_map[i] < 0 else "0"

    gamma_num = int(gamma, 2)
    beta_num = int(beta, 2)

    return gamma_num * beta_num


def part_2(lines):
    return oxygen(lines) * co2(lines)

def oxygen(lines):
    length = len(lines[0].strip())
    filtered_lines = [l.strip() for l in lines]
    for i in range(length):
        msb = 0
        for l in filtered_lines:
            line = l.strip()
            msb += 1 if line[i] == '1' else -1
            
        filtered_lines = list(filter(lambda l: l.strip()[i] == ("1" if msb >= 0 else "0"), filtered_lines))
        if len(filtered_lines) == 1:
            return int(filtered_lines[0], 2)

def co2(lines):
    length = len(lines[0].strip())
    filtered_lines = [l.strip() for l in lines]
    for i in range(length):
        msb = 0
        for l in filtered_lines:
            line = l.strip()
            msb += 1 if line[i] == '1' else -1
            
        filtered_lines = list(filter(lambda l: l.strip()[i] == ("1" if msb < 0 else "0"), filtered_lines))
        if len(filtered_lines) == 1:
            return int(filtered_lines[0], 2)
    
if __name__ == "__main__":
    with open("3.input") as f:
        lines = f.readlines()
        print("part 1:", part_1(lines))
        print("part 2:", part_2(lines))
