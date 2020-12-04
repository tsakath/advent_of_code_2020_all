use std::{
    env,
    fs::File,
    io::{prelude::*, BufReader},
};

fn main() {
    let mut args = env::args();
    args.next();
    let filename = match args.next() {
        Some(arg) => arg,
        None => String::from("you must provide a filename"),
    };
    let file = File::open(filename).expect("no such file");
    let buff = BufReader::new(file);
    let lst: Vec<i32> = buff.lines()
        .map(|line| line.expect("could not parse line")
            .parse()
            .unwrap()
        )
        .collect();

    let len = lst.len();

    'part_1: for (idx, i) in lst.iter().enumerate() {
        for j in lst[idx+1..len].iter() {
            if i + j == 2020 {
                print!("{} * {} = {}\n", i, j, i*j);
                break 'part_1
            }
        }
    }

    'part_2: for (idx, i) in lst.iter().enumerate() {
        for (idx2, j) in lst[idx+1..lst.len()].iter().enumerate() {
            for  n in lst[idx2+1..len].iter() {
                if i + j + n == 2020 {
                    print!("{} * {} * {} = {}\n", i, j, n,  i*j*n);
                    break 'part_2
                }
            }
        }
    }
}
