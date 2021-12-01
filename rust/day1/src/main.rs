use std::fs;

fn main() {
    println!("Hello, world!");

    let f = fs::read_to_string("1.input").expect("Unable to open file!");
    let lines = f.split("\n");
    let mut previous_num = 0;
    let mut increases = 0;
    for line in lines {
        let num: i32 = line.parse().unwrap();
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
