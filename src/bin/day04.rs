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

    let mut data: Vec<(i32, Vec<i32>, Vec<i32>)> = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        let round = parse_data(&line, index as i32);
        data.push(round);
    }

    println!("-- PART 1 --");
    part1(&data);
    println!("-- PART 2 --");
    part2(&data);
    Ok(())
}

fn parse_data(line: &str, card_number: i32) -> (i32, Vec<i32>, Vec<i32>) {
    let re = Regex::new(r"([\d\s]+)\s*\|\s*([\d\s]+)").unwrap();
    if let Some(captures) = re.captures(line) {
        let first_part = captures[1].split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let second_part = captures[2].split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        (card_number, first_part, second_part)
    }
    else {
        eprintln!("Error parsing line: {}", line);
        (card_number, Vec::new(), Vec::new())
    }
}

fn part1(data: &[(i32, Vec<i32>, Vec<i32>)]) {
    let mut sum = 0;
    for (_card_number, card, numbers) in data {
        let mut term = 0;
        for num in numbers {
            if card.contains(num) {
                if term == 0 {
                    term = 1;
                }
                else {
                    term *= 2;
                }
            }
        }
        sum += term;
    }
    println!("sum: {}", sum);
}

fn part2(data: &[(i32, Vec<i32>, Vec<i32>)]) {
    let mut stack: Vec<(i32, Vec<i32>, Vec<i32>)> = Vec::new();
    let mut sum = 0;
    for i in 0..data.len() {
        stack.push(data[i].clone());  
    }

    while let Some((card_number, card, numbers)) = stack.pop() {
        let mut i = 0;
        for num in &numbers {
            if card.contains(num) {
                i += 1;
            }
        }
        for j in 1..i + 1 {
            stack.push(data[card_number as usize + j].clone());
        }
        sum += 1;
    }
    println!("sum: {}", sum);
}

