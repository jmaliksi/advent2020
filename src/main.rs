use std::str::FromStr;
use std::fs::{self, File};
use std::io::{prelude::*, Result, BufReader};
use std::fmt::Debug;
use std::convert::TryInto;

fn main() {
    println!("day1a: {}", day1("data/day1.txt"));
    println!("day1b: {}", day1b("data/day1.txt"));

    println!("day2: {}", day2("data/day2.txt"));
    println!("day2b: {}", day2b("data/day2.txt"));

    println!("day3: {}", day3("data/day3.txt", 3, 1));
    println!("day3b: {}", day3b());
}


fn load_map(path: &str) -> Vec<Vec<char>> {
    fs::read_to_string(path).expect("bluh").lines().map(|l| l.chars().collect()).collect()
}

fn day3(path: &str, slope_x: usize, slope_y: usize) -> i32 {
    let map = load_map(path);
    let mut toboggan = 0;
    let mut trees = 0;
    for (i, row) in map.iter().enumerate() {
        if i % slope_y != 0 {
            continue;
        }
        if row[toboggan % row.len()] == '#' {
            trees += 1;
        }
        toboggan += slope_x;
    }
    return trees;
}

fn day3b() -> i64 {
    let slopes = [
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ];
    let mut trees:i64 = 1;
    for (slope_x, slope_y) in slopes.iter() {
        trees *= i64::from(day3("data/day3.txt", *slope_x, *slope_y));
        println!("{}", trees);
    }

    return trees;
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

        if (test.chars().nth(pos1-1).unwrap() == pattern) ^ (test.chars().nth(pos2-1).unwrap() == pattern) {
            num_valid += 1;
        }
    }
    return num_valid;
}
