use std::fs;

fn main() {
    let f = fs::read_to_string("sample").expect("Unable to open file!");
    let lines = f.split("\n").collect::<Vec<&str>>();

    if lines.len() > 1 {
        let drawings = lines[0]
            .split(',')
            .filter_map(|f| f.parse::<u64>().ok())
            .collect::<Vec<u64>>();
        let mut boards = get_boards(&lines[1..]);

        part_one(&drawings, &mut boards);
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

fn part_one(drawings: &Vec<u64>, boards: &mut Vec<Vec<Vec<u64>>>) {
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
        }
    }
    println!("{:?}", boards);
}