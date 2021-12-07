use std::fs;

fn main() {
    let f = fs::read_to_string("1.input").expect("Unable to open file!");
    let lines = f.split("\n").collect::<Vec<&str>>();
    let mut previous_num = 0;
    let mut increases = 0;
    for i in 0..lines.len() - 2 {
        let num = lines[i].parse::<i32>().unwrap()
            + lines[i + 1].parse::<i32>().unwrap()
            + lines[i + 2].parse::<i32>().unwrap();
        if previous_num == 0 {
            previous_num = num;
            continue;
        }

        if num > previous_num {
            increases += 1;
        }

        previous_num = num;
    }

    println!("{}", increases)
}
