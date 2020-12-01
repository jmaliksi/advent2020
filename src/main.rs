use std::fs::File;
use std::io::{prelude::*, Result, BufReader};

fn main() {
    println!("{}", day2("data/day1.txt"));
}


fn file_to_vec(path: &str) -> Result<Vec<i32>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    Ok(reader.lines().filter_map(Result::ok).map(|r| r.parse::<i32>().unwrap()).collect())
}


fn day1(path: &str) -> i32 {
    let lines = match file_to_vec(path) {
        Err(_why) => panic!("oh no"),
        Ok(lines) => lines,
    };

    let target = 2020;

    for i in lines.iter() {
        for j in lines.iter() {
            if i + j == target {
                return i * j;
            }
        }
    }
    return -1;
}

fn day2(path: &str) -> i32 {
    let lines = match file_to_vec(path) {
        Err(_why) => panic!("oh no"),
        Ok(lines) => lines,
    };

    let target = 2020;

    // it's 200 entries lol
    for i in lines.iter() {
        for j in lines.iter() {
            for k in lines.iter() {
                if i + j + k == target {
                    return i * j * k;
                }
            }
        }
    }
    return -1;
}
