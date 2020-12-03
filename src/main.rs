use std::str::FromStr;
use std::fs::File;
use std::io::{prelude::*, Result, BufReader};
use std::fmt::Debug;
use std::convert::TryInto;

fn main() {
    println!("day1a: {}", day1("data/day1.txt"));
    println!("day1b: {}", day1b("data/day1.txt"));

    println!("day2: {}", day2("data/day2.txt"));
    println!("day2b: {}", day2b("data/day2.txt"));
}


fn file_to_vec<T: FromStr>(path: &str) -> Result<Vec<T>> where <T as FromStr>::Err: Debug {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    Ok(reader.lines().filter_map(Result::ok).map(|r| r.parse().expect("uh oh")).collect())
}


fn day1(path: &str) -> i32 {
    let lines = match file_to_vec::<i32>(path) {
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

fn day1b(path: &str) -> i32 {
    let lines = match file_to_vec::<i32>(path) {
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

fn day2(path: &str) -> i32 {
    let lines = match file_to_vec::<String>(path) {
        Err(_why) => panic!("oh no"),
        Ok(lines) => lines,
    };
    let mut num_valid = 0;
    for line in lines.iter() {
        let tokens = line.split(" ").collect::<Vec<&str>>();
        let range = tokens[0].split("-").collect::<Vec<&str>>();
        let min = range[0].parse::<i32>().unwrap();
        let max = range[1].parse::<i32>().unwrap();
        let pattern = tokens[1].chars().nth(0).unwrap();
        let test = tokens[2];

        let char_count = test.matches(pattern).count();
        if char_count >= min.try_into().unwrap() && char_count <= max.try_into().unwrap() {
            num_valid += 1;
        }
    }
    return num_valid;
}

fn day2b(path: &str) -> i32 {
    let lines = match file_to_vec::<String>(path) {
        Err(_why) => panic!("oh no"),
        Ok(lines) => lines,
    };
    let mut num_valid = 0;
    for line in lines.iter() {
        let tokens = line.split(" ").collect::<Vec<&str>>();
        let range = tokens[0].split("-").collect::<Vec<&str>>();
        let pos1:usize = range[0].parse::<i32>().unwrap().try_into().unwrap();
        let pos2:usize = range[1].parse::<i32>().unwrap().try_into().unwrap();
        let pattern = tokens[1].chars().nth(0).unwrap();
        let test = tokens[2];

        println!("{} {} {}", pos1, pos2, test);
        if (test.chars().nth(pos1-1).unwrap() == pattern) ^ (test.chars().nth(pos2-1).unwrap() == pattern) {
            num_valid += 1;
        }
    }
    return num_valid;
}
