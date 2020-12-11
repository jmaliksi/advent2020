use std::str::FromStr;
use std::fs::{self, File};
use std::io::{self, prelude::*, BufReader};
use std::fmt::Debug;
use std::convert::TryInto;
use regex::Regex;
use std::collections::{HashSet, HashMap, VecDeque};
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

    println!("day9: {}", day9("data/day9.txt"));
    println!("day9b: {}", day9b("data/day9.txt"));

    println!("day10: {}", day10("data/day10.txt"));
    println!("day10b: {}", day10b("data/day10.txt"));

    println!("");
    println!("day11: {}", day11("data/day11.txt"));
    println!("day11b: {}", day11b("data/day11.txt"));
}

struct Seating {
    seats: Vec<Vec<char>>,
}

impl Seating {
    fn new(path: &str) -> Seating {
        Seating {
            seats: load_map(path),
        }
    }

    fn copy(&self) -> Seating {
        let mut cp = Vec::new();
        for row in self.seats.iter() {
            let mut new_row = Vec::new();
            for seat in row.iter().cloned() {
                new_row.push(seat);
            }
            cp.push(new_row);
        }
        return Seating{seats:cp};
    }

    fn is_occupied(&self, row: usize, col: usize) -> bool {
        if row >= self.seats.len() || col >= self.seats[row].len() {
            return false;
        }
        return self.seats[row][col] == '#';
    }

    fn count_occupied_neighbors(&self, row: usize, col: usize) -> u8 {
        let mut count = 0;
        for r in 0..3 {
            for c in 0..3 {
                if r == 1 && c == 1 {
                    continue;
                }
                if row + r == 0 || col + c == 0 {
                    continue
                }
                if self.is_occupied(row + r - 1, col + c - 1) {
                    count += 1;
                }
            }
        }
        return count;
    }

    fn simulate(&mut self) -> bool {
        // returns true if stable
        let mut stable = true;
        let frame = self.copy();
        for (r, row) in frame.seats.iter().enumerate() {
            for (c, seat) in row.iter().enumerate() {
                if *seat == '.' {
                    continue;
                }
                let count = frame.count_occupied_neighbors(r, c);
                if *seat == 'L' && count == 0 {
                    self.seats[r][c] = '#';
                    stable = false;
                }
                if *seat == '#' && count >= 4 {
                    self.seats[r][c] = 'L';
                    stable = false;
                }
            }
        }
        return stable;
    }

    fn simulate_lines(&mut self) -> bool {
        // returns true if stable
        let mut stable = true;
        let frame = self.copy();
        for (r, row) in frame.seats.iter().enumerate() {
            for (c, seat) in row.iter().enumerate() {
                if *seat == '.' {
                    continue;
                }
                let count = frame.count_occupied_lines(r, c);
                if *seat == 'L' && count == 0 {
                    self.seats[r][c] = '#';
                    stable = false;
                }
                if *seat == '#' && count >= 5 {
                    self.seats[r][c] = 'L';
                    stable = false;
                }
            }
        }
        return stable;
    }

    fn count_occupied_lines(&self, row: usize, col: usize) -> u8 {
        let mut count = 0;
        for dx in &['U', 'C', 'D'] {
            for dy in &['U', 'C', 'D'] {
                if *dx == 'C' && *dy == 'C' {
                    continue;
                }
                if self.is_line_occupied(row, col, *dx, *dy) {
                    count += 1;
                }
            }
        }
        return count;
    }

    fn is_line_occupied(&self, row: usize, col: usize, dx: char, dy: char) -> bool {
        let mut r = row;
        let mut c = col;
        loop {
            r = match dx {
                'U' => r + 1,
                'C' => r,
                'D' if r == 0 => break,
                'D' => r - 1,
                _ => panic!("what"),
            };
            c = match dy {
                'U' => c + 1,
                'C' => c,
                'D' if c == 0 => break,
                'D' => c - 1,
                _ => panic!("what"),
            };
            if r >= self.seats.len() || c >= self.seats[r].len() {
                return false;
            }
            match self.seats[r][c] {
                'L' => return false,
                '#' => return true,
                _ => continue,
            }
        }
        return false;
    }

    fn count_occupied_seats(&self) -> u32 {
        let mut count = 0;
        for row in self.seats.iter() {
            for seat in row.iter() {
                if *seat == '#' {
                    count += 1;
                }
            }
        }
        return count;
    }
}

fn day11(path: &str) -> u32 {
    let mut seats = Seating::new(path);
    loop {
        let stable = seats.simulate();
        if stable {
            break;
        }
    }
    return seats.count_occupied_seats();
}

fn day11b(path: &str) -> u32 {
    let mut seats = Seating::new(path);
    loop {
        let stable = seats.simulate_lines();
        if stable {
            break;
        }
    }
    return seats.count_occupied_seats();
}

fn day10b(path: &str) -> u64 {
    let mut adapters = file_to_vec::<u32>(path).expect("bluh");
    adapters.sort();
    adapters.insert(0, 0);
    let mut path_count = HashMap::new();
    path_count.insert(adapters.last().unwrap(), 1);

    for adapter in adapters.iter().rev() {
        for jolt in &[1, 2, 3] {
            let cur_path_count = match path_count.get(adapter) {
                None => 0,
                Some(v) => *v,
            };
            path_count.insert(
                adapter,
                match path_count.get(&(adapter + jolt)) {
                    None => cur_path_count,
                    Some(v) => cur_path_count + v,
                }
            );
        }
    }

    return *path_count.get(adapters.first().unwrap()).unwrap();
}

fn day10(path: &str) -> u32 {
    let mut adapters = file_to_vec::<u32>(path).expect("bluh");
    adapters.sort();
    let mut last_adapter = 0;
    let mut ones = 0;
    let mut threes = 1;
    for adapter in adapters.iter().cloned() {
        match adapter - last_adapter {
            1 => ones += 1,
            3 => threes += 1,
            _ => (),
        }
        last_adapter = adapter;
    }
    return ones * threes;
}

fn day9b(path: &str) -> i64 {
    let target = day9(path);
    let f = fs::read_to_string(path).expect("bluh");
    let lines = f.lines().map(|s| s.parse::<i64>().expect("no")).collect::<Vec<_>>();
    let mut contiguous = Vec::<i64>::new();

    for i in 0..lines.len() {
        for n in lines[i..].iter().cloned() {
            let sum:i64 = contiguous.iter().cloned().sum();
            if sum == target {
                contiguous.sort();
                return contiguous.first().unwrap() + contiguous.last().unwrap();
            }
            if sum > target {
                contiguous.clear();
                break;
            }
            contiguous.push(n);
        }
    }
    return -1;
}



fn day9(path: &str) -> i64 {
    let f = fs::read_to_string(path).expect("bluh");
    let lines = f.lines().map(|s| s.parse::<i64>().expect("no")).collect::<Vec<_>>();
    let mut q = VecDeque::new();
    for n in lines[..25].iter() {
        q.push_front(n);
    }
    for target in lines[25..].iter() {
        let mut found = false;
        for a in q.iter() {
            for b in q.iter() {
                if (*a) + (*b) == (*target) {
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
        if !found {
            return *target;
        }
        q.push_front(target);
        q.truncate(25);
    }
    return 0;
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
