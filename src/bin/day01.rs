use std::{env, fs::File, io::{self, BufRead}};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2  {
        eprintln!("Usage: <program> <filename>");
        std::process::exit(1);
    }

    let input_file = File::open(&args[1])?;
    let reader = io::BufReader::new(input_file);
    
    let mut data: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let row = line;
        data.push(row);
    }

    println!("-- PART 1 --");
    part1(&data);
    println!("-- PART 2 --");
    part2(&data);
    Ok(())
}

fn part1(data: &Vec<String>) {
    let mut sum: i32 = 0;

    let mut ch1 = '-';
    let mut ch2 = '-';
    for str in data {
        for c in str.chars() {
            if c.is_numeric() {
                ch1 = c;
                break;
            }
        }

        for c in str.chars().rev() {
            if c.is_numeric() {
                ch2 = c;
                break;
            }
        }

        if ch1 != '-' && ch2 != '-' {
            let str = format!("{}{}", ch1, ch2);
            let term:i32 = str.parse().expect("Failed to parse string to int32");
            sum += term;

        }
    }
    println!("sum: {}", sum);
}

fn part2(data: &Vec<String>) {
    let mut sum: i64 = 0;

    let mut strs: Vec<String> = Vec::with_capacity(data.len());
    for str in data {
        let mut result = str.to_string();
        result = result.replace("one", "o1e");
        result = result.replace("two", "t2o");
        result = result.replace("three", "t3e");
        result = result.replace("four", "f4r");
        result = result.replace("five", "f5e");
        result = result.replace("six", "s6x");
        result = result.replace("seven", "s7n");
        result = result.replace("eight", "e8t");
        result = result.replace("nine", "n9e");
        strs.push(result.to_string());
    }
    
    let mut ch1 = '-';
    let mut ch2 = '-';
    for str in strs {
        for c in str.chars() {
            if c.is_numeric() {
                ch1 = c;
                break;
            }
        }

        for c in str.chars().rev() {
            if c.is_numeric() {
                ch2 = c;
                break;
            }
        }

        if ch1 != '-' && ch2 != '-' {
            let str = format!("{}{}", ch1, ch2);
            let term:i64 = str.parse().expect("Failed to parse string to int32");
            sum += term;

        }
    }
    println!("sum: {}", sum);
}
