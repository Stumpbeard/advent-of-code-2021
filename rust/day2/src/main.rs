use std::fs;

fn main() {
    let f = fs::read_to_string("2.input").expect("Unable to open file!");
    let lines = f
        .split("\n")
        .map(|s| s.split(" ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    println!("{}", part_one(&lines));
    println!("{}", part_two(lines));
}

fn part_one(instructions: &Vec<Vec<&str>>) -> i32 {
    let mut forward = 0;
    let mut depth = 0;
    for line in instructions {
        let num: i32 = line[1].parse().unwrap();
        let direction = line[0];
        match direction {
            "forward" => forward += num,
            "down" => depth += num,
            "up" => depth -= num,
            _ => println!("not a direction"),
        }
    }

    forward * depth
}

fn part_two(instructions: Vec<Vec<&str>>) -> i32 {
    let mut forward = 0;
    let mut aim = 0;
    let mut depth = 0;
    for line in instructions {
        let num: i32 = line[1].parse().unwrap();
        let direction = line[0];
        match direction {
            "forward" => {
                forward += num;
                depth += aim * num
            }
            "down" => aim += num,
            "up" => aim -= num,
            _ => println!("not a direction"),
        }
    }

    forward * depth
}
