use std::str::FromStr;
use std::fs::{self, File};
use std::io::{self, prelude::*, BufReader};
use std::fmt::Debug;
use std::convert::TryInto;
use regex::Regex;
use std::collections::{HashSet, HashMap};
use std::result::Result;

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

    println!("day6: {}", day6("data/day6.txt"));
    println!("day6b: {}", day6b("data/day6.txt"));

    println!("day7: {}", day7("data/day7.txt"));
    println!("day7b: {}", day7b("data/day7.txt"));

    println!("day8: {}", day8("data/day8.txt"));
    println!("day8b: {}", day8b("data/day8.txt"));
}

fn day8b(path: &str) -> i32 {
    let f = fs::read_to_string(path).expect("bluh");
    let instructions = f.lines().map(|s| {
        let tokens = s.split(" ").into_iter().collect::<Vec<&str>>();
        let inst = tokens[0];
        let sign = tokens[1].chars().nth(0).expect("no");
        let mut val = tokens[1][1..].parse::<i32>().expect("oh");
        if sign == '-' {
            val *= -1;
        }
        return (inst, val);
    }).collect::<Vec<(&str, i32)>>();

    let mut accumulator = 0;
    for (i, inst) in instructions.iter().enumerate() {
        if inst.0 == "acc" {
            continue;
        }
        let mut copy = instructions.to_vec();
        copy[i] = (
            match inst.0 {
                "nop" => "jmp",
                "jmp" => "nop",
                _ => panic!("what"),
            },
            inst.1
        );
        let res = execute_instructions(&copy);
        if res.is_ok() {
            accumulator = res.unwrap();
        }
    }
    return accumulator;
}

fn execute_instructions(instructions: &Vec<(&str, i32)>) -> Result<i32, &'static str> {
    let mut accumulator = 0;
    let mut executed = HashSet::new();
    let mut execution_index = 0;
    while execution_index < instructions.len() {
        if executed.contains(&execution_index) {
            return Err("no");
        }
        let line = instructions[execution_index];
        executed.insert(execution_index);
        match line.0 {
            "acc" => {
                accumulator += line.1;
                execution_index += 1;
            },
            "nop" => execution_index += 1,
            "jmp" => execution_index = ((execution_index as i32) + line.1) as usize,
            _ => (),
        }
    }

    Ok(accumulator)
}

fn day8(path: &str) -> i32 {
    let mut accumulator = 0;
    let f = fs::read_to_string(path).expect("bluh");
    let instructions = f.lines().map(|s| {
        let tokens = s.split(" ").into_iter().collect::<Vec<&str>>();
        let inst = tokens[0];
        let sign = tokens[1].chars().nth(0).expect("no");
        let mut val = tokens[1][1..].parse::<i32>().expect("oh");
        if sign == '-' {
            val *= -1;
        }
        return (inst, val);
    }).collect::<Vec<(&str, i32)>>();

    let mut executed = HashSet::new();
    let mut execution_index = 0;
    while execution_index < instructions.len() {
        if executed.contains(&execution_index) {
            break;
        }
        let line = instructions[execution_index];
        executed.insert(execution_index);
        match line.0 {
            "acc" => {
                accumulator += line.1;
                execution_index += 1;
            },
            "nop" => execution_index += 1,
            "jmp" => execution_index = ((execution_index as i32) + line.1) as usize,
            _ => (),
        }
    }

    return accumulator;
}

fn contained_bags(bags: &HashMap<String, Vec<String>>, check: &str) -> u32 {
    if check == "no other bag" {
        return 0;
    }
    let bag = &check[2..];
    let mut total:u32 = check.chars().nth(0).unwrap().to_digit(10).unwrap().into();
    let count = total;

    let children = bags.get(bag).unwrap();
    for child in children.iter() {
        total += count * contained_bags(bags, child);
    }
    return total;
}

fn day7b(path: &str) -> u32 {
    let bags = load_bags(path);
    return contained_bags(&bags, "1 shiny gold bag") - 1;
}

fn day7(path: &str) -> i32 {
    let bags = load_bags(path);
    let mut to_visit = Vec::new();
    let mut can_contain = HashSet::new();
    to_visit.push("shiny gold".to_string());
    while !to_visit.is_empty() {
        let check = to_visit.pop().unwrap();
        for (parent, children) in bags.iter() {
            for child in children.iter() {
                if child.contains(&check) {
                    to_visit.push(parent.to_string());
                    can_contain.insert(parent.to_string());
                }
            }
        }
    }
    return can_contain.len().try_into().unwrap();
}

fn load_bags(path: &str) -> HashMap<String, Vec<String>> {
    let mut res = HashMap::new();
    for line in fs::read_to_string(path).expect("bluh").lines() {
        let cleaned = line.replace("bags", "bag").replace(".", "");
        let tokens = cleaned.split(" contain ").into_iter().collect::<Vec<&str>>();
        res.insert(
            tokens[0].to_string(),
            tokens[1].split(", ").map(|s| s.to_string()).collect::<Vec<String>>()
        );
    }
    return res;
}

fn day6b(path: &str) -> i32 {
    let answers = load_map(path);
    let mut group = HashSet::new();

    let mut yes = 0;

    let mut new_group = true;
    for answer in answers.iter() {
        if answer.is_empty() {
            yes += group.len();
            group.clear();
            new_group = true;
        } else {
            if new_group {
                for i in answer.iter().cloned() {
                    group.insert(i);
                }
                new_group = false;
            } else {
                let mut g = HashSet::new();
                for i in answer.iter().cloned() {
                    g.insert(i);
                }
                let temp = group.iter().cloned().collect::<HashSet<char>>();
                let intx = g.intersection(&temp);
                group.clear();
                for i in intx {
                    group.insert(*i);
                }
            }
        }
    }
    yes += group.len();

    return yes.try_into().unwrap();
}

fn day6(path: &str) -> i32 {
    let answers = load_map(path);
    let mut group = HashSet::new();

    let mut yes = 0;

    for answer in answers.iter() {
        for ans in answer.iter() {
            group.insert(ans);
        }
        if answer.is_empty() {
            yes += group.len();
            group.clear();
        }
    }
    yes += group.len();

    return yes.try_into().unwrap();
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

fn file_to_vec<T: FromStr>(path: &str) -> io::Result<Vec<T>> where <T as FromStr>::Err: Debug {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    Ok(reader.lines().filter_map(io::Result::ok).map(|r| r.parse().expect("uh oh")).collect())
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
