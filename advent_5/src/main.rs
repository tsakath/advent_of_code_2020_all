use std::{
    env,
    fs,
};

fn main() {
    let mut args = env::args();
    args.next();
    let filename = args.next().expect("no such file");
    let contents = fs::read_to_string(filename).expect("cannot read file content");

    let mut part_1 = 0;
    let mut part_2 = 0;

    let mut lst = vec![];

    for line in contents.lines() {
        let mut id = 0;

        let mut mid = 64;
        for c in line[..7].chars() {
            if c == 'B' {
                id += mid;
            }
            mid = mid >> 1;
        }
        id = id << 3;

        let mut mid = 4;
        for c in line[7..].chars() {
            if c == 'R' {
                id += mid;
            }
            mid = mid >> 1;
        }

        if id > part_1 {
            part_1 = id;
        }

        lst.push(id);
    }

    lst.sort_unstable();
    for w in lst.windows(2) {
        if w[1] - w[0] == 2 {
            part_2 = w[0] + 1;
            break;
        }
    }

    println!("{}", part_1);
    println!("{}", part_2);
}
