use std::fs;
use std::collections::HashMap;

fn main() {
    let f = fs::read_to_string("3.input").expect("Unable to open file!");
    let lines = f
        .split("\n")
        .collect::<Vec<&str>>();

    println!("{}", part_one(&lines));
    // println!("{}", part_two(lines));
}

fn part_one(instructions: &Vec<&str>) -> i64 {
    let mut freq_map: HashMap<usize, i64> = HashMap::new();
    for line in instructions {
        for i in 0..12 {
            if line.chars().nth(i).unwrap()
            .to_digit(10).unwrap() > 0 {
                let v = freq_map.entry(i).or_insert(0);
                *v += 1;
            } else {
                let v = freq_map.entry(i).or_insert(0);
                *v += -1;
            }
        }
    }

    let base: i64 = 2;
    let mut gamma_num = 0;
    let mut beta_num = 0;
    for i in 0..12 {
        if freq_map[&(11-i)] > 0 {
            gamma_num += base.pow(i.try_into().unwrap());
        } else {
            beta_num += base.pow(i.try_into().unwrap());
        }
    }

    gamma_num * beta_num
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
