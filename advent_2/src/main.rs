use std::{
    env,
    fs,
};

fn main() {
    let mut args = env::args();
    args.next();
    let filename = match args.next() {
        Some(arg) => arg,
        None => panic!("no such file"),
    };
    let contents = fs::read_to_string(filename).unwrap();

    let mut part_1 = 0;
    let mut part_2 = 0;
    for line in contents.lines() {
        let mut entry = line
            .split(|l| l == '-' || l == ' ' || l == ':')
            .filter(|x| !x.is_empty());
        let min = entry.next().unwrap().parse().unwrap();
        let max = entry.next().unwrap().parse().unwrap();
        let value = entry.next().unwrap();
        let password = entry.next().unwrap();

        // Part 1
        let count = password.matches(value).count();
        if count <= max && count >= min {
            part_1 += 1;
        }

        // Part 2
        let mut first = false;
        let mut second = false;
        if let Some(v) = password.chars().nth(min-1) {
            first = v.to_string() == value;
        }
        if let Some(v) = password.chars().nth(max-1) {
            second = v.to_string() == value;
        }
        if (first && !second) || (!first && second) {
            part_2 += 1;
        }
    }
    println!("{}", part_1);
    println!("{}", part_2);
}
