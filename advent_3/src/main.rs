use std::{
    env,
    fs,
};

struct Step {
    x: usize,
    y: usize
}

fn main() {
    let mut args = env::args();
    args.next();
    let filename = match args.next() {
        Some(arg) => arg,
        None => panic!("no such file"),
    };
    let contents = fs::read_to_string(filename).unwrap();

    // Part 1
    let part_1 = count_trees(&contents, &Step{x: 2, y: 1});
    println!("{}", part_1);

    // Part 2
    let steps = vec![Step{x: 1, y: 1}, Step{x: 3, y: 1}, Step{x: 5, y: 1}, Step{x: 7, y: 1}, Step{x: 1, y: 2}];
    let part_2 = steps
        .iter()
        .map(|s| count_trees(&contents, s))
        .fold(1, |acc, x| acc * x);
    println!("{}", part_2);
}

fn count_trees(contents: &str, step: &Step) -> usize {
    let mut count = 0;
    let mut x: usize = step.x;
    for line in contents.lines().skip(step.y).step_by(step.y) {
        if let Some('#') = line.chars().nth(x) {
            count += 1;
        };
        x = (x + step.x) % line.len();
    }
    count
}
