def part_1(drawings, boards):
    for num in drawings:
        for board in boards:
            for row in board:
                for i, place in enumerate(row):
                    if place == num:
                        row[i] = True
                        if check_win(board):
                            return calculate_score(board, num)


def part_2(drawings, boards):
    for num in drawings:
        for board in boards:
            for row in board:
                for i, place in enumerate(row):
                    if place == num:
                        row[i] = True
                        if check_win(board):
                            if len(boards) == 1:
                                return calculate_score(board, num)
                            else:
                                return part_2(drawings, [b for b in boards if b != board])


def check_win(board):
    for row in board:
        row_check = list(filter(lambda n: n == True, row))
        if len(row_check) == 5:
            return True

    for i in range(len(board[0])):
        column = [row[i] for row in board]
        column_check = list(filter(lambda n: n == True, column))
        if len(column_check) == 5:
            return True

    return False

def calculate_score(board, num):
    unmarked_sum = 0
    for row in board:
        for column in row:
            if column != True:
                unmarked_sum += int(column)

    return unmarked_sum * int(num)

def get_boards(lines):
    boards = []
    board = []
    for line in lines:
        row = line.split()
        if len(row) != 5:
            continue

        board.append(row)
        if len(board) == 5:
            boards.append(board)
            board = []

    return boards

if __name__ == "__main__":
    with open("input") as f:
        lines = [l.strip() for l in f.readlines()]
        drawings = lines[0].split(',')
        boards = get_boards(lines[1:])

        print("part 1:", part_1(drawings, boards))
        print("part 2:", part_2(drawings, boards))