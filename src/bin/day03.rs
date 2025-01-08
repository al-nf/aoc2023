use std::{env, fs::File, io::{self, BufRead}};
use regex::Regex;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: <program> <filename>");
        std::process::exit(1);
    }

    let input_file = File::open(&args[1])?;
    let reader = io::BufReader::new(input_file);
    
    let mut data: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let row = line.chars().collect();
        data.push(row);
    }

    println!("-- PART 1 --");
    part1(&data);
    println!("-- PART 2 --");
    part2(&data);
    Ok(())
}

fn part1(data: &[Vec<char>]) {
    let mut parts: Vec<(i32, (usize, usize), usize)> = Vec::new();

    let re = Regex::new(r"\d+").unwrap();

    for (row_index, row) in data.iter().enumerate() {
        let s: String = row.iter().collect();
        for mat in re.find_iter(&s) {
            let number = mat.as_str().parse::<i32>().unwrap();
            let col_index = mat.start();
            let length = mat.end() - mat.start();
            parts.push((number, (row_index, col_index), length));
        }
    }

    let mut sum = 0;

    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),          (0, 1), 
        (1, -1), (1, 0), (1, 1), 
    ];

    for &(number, (row, col), length) in &parts {
        let mut is_adjacent_to_symbol = false;

        for digit_offset in 0..length {
            let digit_col = col + digit_offset;

            for &(dr, dc) in &directions {
                let new_row = row as isize + dr;
                let new_col = digit_col as isize + dc;

                if new_row >= 0 && new_row < data.len() as isize && new_col >= 0 && new_col < data[0].len() as isize {
                    let neighbor = data[new_row as usize][new_col as usize];
                    if !neighbor.is_ascii_digit() && neighbor != '.' {
                        is_adjacent_to_symbol = true;
                        break;
                    }
                }
            }

            if is_adjacent_to_symbol {
                break;
            }
        }

        if is_adjacent_to_symbol {
            sum += number;
        }
    }

    println!("sum: {}", sum);
}

fn part2(data: &[Vec<char>]) {
    let re = Regex::new(r"\d+").unwrap(); 

    let mut number_map: Vec<Vec<Option<i32>>> = vec![vec![None; data[0].len()]; data.len()];

    for (row_index, row) in data.iter().enumerate() {
        let s: String = row.iter().collect();
        for mat in re.find_iter(&s) {
            let number = mat.as_str().parse::<i32>().unwrap();
            let col_start = mat.start();
            let col_end = mat.end();

            for col_index in col_start..col_end {
                number_map[row_index][col_index] = Some(number);
            }
        }
    }

    let mut gears: Vec<(usize, usize)> = Vec::new();
    for i in 0..data.len() {
        for j in 0..data[0].len() {
            if data[i][j] == '*' {
                gears.push((i, j));
            }
        }
    }

    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),          (0, 1),
        (1, -1), (1, 0), (1, 1),
    ];

    let mut sum = 0;

    for (gear_row, gear_col) in gears {
        let mut adjacent_numbers: std::collections::HashSet<i32> = std::collections::HashSet::new();

        for &(dr, dc) in &directions {
            let new_row = gear_row as isize + dr;
            let new_col = gear_col as isize + dc;

            if new_row >= 0 && new_row < data.len() as isize && new_col >= 0 && new_col < data[0].len() as isize {
                if let Some(number) = number_map[new_row as usize][new_col as usize] {
                    adjacent_numbers.insert(number);
                }
            }
        }
        if adjacent_numbers.len() == 2 {
            let numbers: Vec<i32> = adjacent_numbers.into_iter().collect();
            let gear_ratio = numbers[0] * numbers[1];
            sum += gear_ratio;
        }
    }

    println!("sum: {}", sum);
}


