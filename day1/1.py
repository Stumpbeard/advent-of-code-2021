with open('1.input') as f:
    previous_sum = None
    increases = 0
    lines = f.readlines()
    for i in range(len(lines) - 2):
        if not previous_sum:
            previous_sum = int(lines[i]) + int(lines[i+1]) + int(lines[i+2])
            continue

        sum = int(lines[i]) + int(lines[i+1]) + int(lines[i+2])
        if sum > previous_sum:
            increases += 1

        previous_sum = sum


    print(increases)