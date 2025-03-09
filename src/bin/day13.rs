use std::{env, fs::File, io::{self, BufRead}};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: <program> <filename>");
        std::process::exit(1);
    }

    let input_file = File::open(&args[1])?;
    let reader = io::BufReader::new(input_file);

    let mut data: Vec<Vec<Vec<char>>> = Vec::new(); 
    let mut map: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line?; 
        if !line.is_empty() {
            map.push(line.chars().collect());
        } else {
            if !map.is_empty() {
                data.push(map.clone());
                map.clear();
            }
        }
    }
    if !map.is_empty() {
        data.push(map);
    }

    println!("-- PART 1 --");
    part1(&data);
    println!("-- PART 2 --");
    part2(&data);
    Ok(())
}

fn part1(data: &[Vec<Vec<char>>]) {
    let mut sum: u64 = 0;

    let is_horizontal_reflection = |pattern: &[Vec<char>], row: usize| -> bool {
        let mut lower = row;
        let mut upper = row + 1;
        while lower < pattern.len() && upper < pattern.len() {
            if pattern[lower] != pattern[upper] {
                return false;
            }
            if lower == 0 {
                break;
            }
            lower -= 1;
            upper += 1;
        }
        true
    };

    let is_vertical_reflection = |pattern: &[Vec<char>], col: usize| -> bool {
        let mut lower = col;
        let mut upper = col + 1;
        while upper < pattern[0].len() {
            for row in pattern.iter() {
                if row[lower] != row[upper] {
                    return false;
                }
            }
            if lower == 0 {
                break;
            }
            lower -= 1;
            upper += 1;
        }
        true
    };

    for pattern in data {
        for i in 0..pattern.len() - 1 {
            if is_horizontal_reflection(pattern, i) {
                sum += 100 * (i + 1) as u64;
            }
        }
        for i in 0..pattern[0].len() - 1 {
            if is_vertical_reflection(pattern, i) {
                sum += (i + 1) as u64;
            }
        }
    }
    
    println!("sum: {}", sum);
}

fn part2(data: &[Vec<Vec<char>>]) {
}

