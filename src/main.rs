use std::str::FromStr;
use std::fs::{self, File};
use std::io::{prelude::*, Result, BufReader};
use std::fmt::Debug;
use std::convert::TryInto;
use regex::Regex;

fn main() {
    println!("day1a: {}", day1("data/day1.txt"));
    println!("day1b: {}", day1b("data/day1.txt"));

    println!("day2: {}", day2("data/day2.txt"));
    println!("day2b: {}", day2b("data/day2.txt"));

    println!("day3: {}", day3("data/day3.txt", 3, 1));
    println!("day3b: {}", day3b());

    println!("day4: {}", day4("data/day4.txt"));
    println!("day4b: {}", day4b("data/day4.txt"));

    println!("day5: {}", day5("data/day5.txt"));
    println!("day5b: {}", day5b("data/day5.txt"));
}

fn day5b(path: &str) -> i32 {
    let seats = load_map(path);
    let mut seat_ids = Vec::new();

    for seat in seats {
        let row = search_rows(&seat[0..7], 128);
        let col = search_rows(&seat[7..], 8);
        seat_ids.push(row * 8 + col);
    }
    seat_ids.sort();
    let mut last_seat = seat_ids.pop().unwrap();
    while !seat_ids.is_empty() {
        let next_seat = seat_ids.pop().unwrap();
        if last_seat - 1 != next_seat {
            return last_seat - 1;
        }
        last_seat = next_seat;
    }
    return -1;
}

fn day5(path: &str) -> i32 {
    let seats = load_map(path);
    let mut highest = -1;

    for seat in seats {
        let row = search_rows(&seat[0..7], 128);
        let col = search_rows(&seat[7..], 8);
        if row * 8 + col > highest {
            highest = row * 8 + col;
        }
    }
    return highest;
}

fn search_rows(bsp: &[char], max: i32) -> i32 {
    let mut top = max;
    let mut bottom = 0;
    let mut row = top / 2;
    for pos in bsp {
        if pos == &'F' || pos == &'L' {
            top = row;
            row = (row + bottom) / 2;
        } else {
            bottom = row;
            row = (row + top) / 2;
        }
    }
    return row;
}

fn day4b(path: &str) -> i32 {
    let lines = match file_to_vec::<String>(path) {
        Err(_why) => panic!("oh"),
        Ok(lines) => lines,
    };

    let mut valid = 0;
    let hcl_regex = Regex::new(r"#[a-f0-9]{6}").unwrap();
    let pid_regex = Regex::new(r"[0-9]{9}").unwrap();

    let mut num_valid_keys = 0;
    for line in lines {
        if line == "" {
            if num_valid_keys == 7 {
                valid += 1;
            }
            num_valid_keys = 0;
            continue;
        }

        for token in line.split(" ") {
            let d = &token.split(":").collect::<Vec<&str>>()[1];

            if (token.starts_with("byr") && d.len() == 4 && d >= &"1920" && d <= &"2002") ||
                (token.starts_with("iyr") && d.len() == 4 && d >= &"2010" && d <= &"2020") ||
                (token.starts_with("eyr") && d.len() == 4 && d >= &"2020" && d <= &"2030") ||
                (token.starts_with("hgt") && ((d.ends_with("cm") && d >= &"150cm" && d <= &"193cm") || (d.ends_with("in") && d >= &"59in" && d <= &"76in"))) ||
                (token.starts_with("hcl") && hcl_regex.is_match(d)) ||
                (token.starts_with("ecl") && (d == &"amb" || d == &"blu" || d == &"brn" || d == &"gry" || d == &"grn" || d == &"hzl" || d == &"oth")) ||
                (token.starts_with("pid") && pid_regex.is_match(d)) {

                num_valid_keys += 1;
            }
        }
    }
    if num_valid_keys == 7 {
        valid += 1;
    }

    return valid
}

fn day4(path: &str) -> i32 {
    let lines = match file_to_vec::<String>(path) {
        Err(_why) => panic!("oh"),
        Ok(lines) => lines,
    };

    let mut valid = 0;
    let fields = [
        "byr",
        "iyr",
        "eyr",
        "hgt",
        "hcl",
        "ecl",
        "pid",
        // "cid",
    ];

    let mut num_valid_keys = 0;

    for line in lines {
        for token in line.split(" ") {
            for field in &fields {
                if token.starts_with(field) {
                    num_valid_keys += 1;
                }
            }
        }
        if line == "" {
            // validate
            if num_valid_keys == fields.len() {
                valid += 1;
            }
            num_valid_keys = 0;
        }
    }
    if num_valid_keys == fields.len() {
        valid += 1;
    }
    return valid;
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
