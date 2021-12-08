use std::{fs, ops::Index};

fn main() {
    let f = fs::read_to_string("sample").expect("Unable to open file!");
    let lines = f.split("\n").collect::<Vec<&str>>();

    if lines.len() > 1 {
        let drawings = lines[0]
            .split(',')
            .filter_map(|f| f.parse::<u64>().ok())
            .collect::<Vec<u64>>();
        let mut boards = get_boards(&lines[1..]);

        println!("part 1: {}", part_one(&drawings, &mut boards));
        println!("part 2: {}", part_two(&drawings, &mut boards, vec![]));
    }
}

fn get_boards(lines: &[&str]) -> Vec<Vec<Vec<u64>>> {
    let mut boards: Vec<Vec<Vec<u64>>> = vec![];
    let mut board: Vec<Vec<u64>> = vec![];
    for &line in lines {
        let row = line
            .split(' ')
            .filter_map(|f| f.parse::<u64>().ok())
            .collect::<Vec<u64>>();
        if row.len() < 5 {
            continue;
        }
        board.push(row);
        if board.len() >= 5 {
            boards.push(board);
            board = vec![];
        }
    }

    boards
}

fn check_win(board: &Vec<Vec<u64>>) -> bool {
    for row in board {
        let row_check = row.iter().filter(|n| **n == 0).collect::<Vec<&u64>>();
        if row_check.len() == 5 {
            return true;
        }
    }

    for i in 0..board[0].len() {
        let mut column = vec![];
        for x in 0..board.len() {
            let row = &board[x];
            column.push(row[i]);
        }
        let column_check = column.iter().filter(|n| **n == 0).collect::<Vec<&u64>>();
        if column_check.len() == 5 {
            return true;
        }
    }

    false
}

fn calculate_score(board: &Vec<Vec<u64>>, num: &u64) -> u64 {
    let mut unmarked_sum = 0;
    for row in board {
        for column in row {
            unmarked_sum += column;
        }
    }

    unmarked_sum * num
}

fn part_one(drawings: &Vec<u64>, boards: &mut Vec<Vec<Vec<u64>>>) -> u64 {
    for num in drawings {
        for x in 0..boards.len() {
            let board = &mut boards[x];
            for y in 0..board.len() {
                let row = &mut board[y];
                for z in 0..row.len() {
                    let place = row[z];
                    if place == *num {
                        row[z] = 0;
                    }
                }
            }
            if check_win(board) {
                return calculate_score(board, num);
            }
        }
    }

    0
}

fn part_two(drawings: &Vec<u64>, boards: &mut Vec<Vec<Vec<u64>>>, mut skips: Vec<usize>) -> u64 {
    let boards_size = boards.len();
    for num in drawings {
        for x in 0..boards.len() {
            if skips.contains(&x) {
                continue;
            }
            let board = &mut boards[x];
            for y in 0..board.len() {
                let row = &mut board[y];
                for z in 0..row.len() {
                    let place = row[z];
                    if place == *num {
                        row[z] = 0;
                    }
                }
            }
            if check_win(board) {
                if boards_size - skips.len() == 1 {
                    return calculate_score(board, num);
                } else {
                    skips.push(x);
                    return part_two(drawings, boards, skips);
                }
            }
        }
    }

    0
}
