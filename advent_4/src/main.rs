use std::{
    env,
    fs,
};

fn main() {
    let mut args = env::args();
    args.next();
    let filename = args.next().expect("no such file");
    let contents = fs::read_to_string(filename).expect("cant read file contents");

    let mut part_1 = 0;
    let mut part_2 = 0;

    let mut info = String::from("");
    for line in contents.lines() {
        if !line.is_empty() {
            info.push_str(format!(" {}", line).as_str());
            continue
        }
        if present(&info) {
            part_1 += 1;
        }
        if present(&info) && valid(&info) {
            part_2 += 1;
        }
        info.clear();
    }

    println!("{}", part_1);
    println!("{}", part_2);
}

fn present(info: &str) -> bool {
    let valid_info = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    for vi in valid_info.iter() {
        if info.matches(vi).count() == 0 {
            return false
        }
    }
    true
}

fn valid(info: &str) -> bool {
    for part in info.split_whitespace().collect::<Vec<&str>>() {
        let mut entry = part.split(":");

        let k = entry.next().unwrap_or("");
        let v = entry.next().unwrap_or("");

        let valid = match k {
            "byr" => {
                let v = v.parse().unwrap_or(0);
                v >= 1920 && v <= 2002
            },
            "iyr" => {
                let v = v.parse().unwrap_or(0);
                v >= 2010 && v <= 2020
            },
            "eyr" =>  {
                let v = v.parse().unwrap_or(0);
                v >= 2020 && v <= 2030
            },
            "hgt" => {
                let (height_number, height_metric) = v.split_at(v.len()-2);
                let height_number = height_number.parse::<u32>().unwrap_or(0);
                let valid= match height_metric {
                    "cm" =>  {
                        if height_number >= 150 && height_number <= 193 {
                            true
                        } else {
                            false
                        }
                    }
                    "in" => {
                        if height_number >= 59 && height_number <= 76 {
                            true
                        } else {
                            false
                        }
                    }
                    _ => false,
                };
                valid
            },
            "hcl" => {
                if !v.starts_with('#') {
                    false
                } else {
                    let (_, p) = v.split_at(1);
                    p.len() == 6 && p.chars().all(|a| a.is_digit(16))
                }
            },
            "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&v),
            "pid" =>  (v.len() == 9 && v.chars().all(|a| a.is_digit(10))),
            "cid" => true,
            _ => false,
        };

        if !valid {
            return false
        }
    }
    true
}
