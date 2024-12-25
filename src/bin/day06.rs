use std::{env, fs::File, io::{self, BufRead}};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: <program> <filename>");
        std::process::exit(1);
    }

    let input_file = File::open(&args[1])?;
    let reader = io::BufReader::new(input_file);
    
    let mut data_part1: Vec<(i32, i32)> = Vec::new();
    let mut data_part2: (i64, i64) = (0, 0);
    let mut time_buffer: Vec<i32> = Vec::new();
    let mut distance_buffer: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.starts_with("Time:") {
            time_buffer = parse_data(&line);
            data_part2.0 = parse_data_alt(&line);
        } else if line.starts_with("Distance:") {
            distance_buffer = parse_data(&line);
            data_part2.1 = parse_data_alt(&line);
        }
    }

    for (time, distance) in time_buffer.into_iter().zip(distance_buffer.into_iter()) {
        data_part1.push((time, distance));
    }

    println!("-- PART 1 --");
    part1(&data_part1);
    println!("-- PART 2 --");
    part2(data_part2);
    Ok(())
}

fn parse_data(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .skip(1)
        .filter_map(|s| s.parse::<i32>().ok())
        .collect()
}

fn parse_data_alt(line: &str) -> i64 {
    line.split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<i64>()
        .unwrap_or(0)
}

fn part1(data: &[(i32, i32)]) {
    let mut ans = 1;
    for (time, distance) in data {
        let mut sum = 0;
        for i in 1..time - 1 {
            if i * (time - i) > *distance {
                sum += 1;
            }
        }
        ans *= sum;
    }
    println!("answer: {}", ans);
}

fn part2(data: (i64, i64)) {
    let (time, distance) = data;

    let discriminant = time * time - 4 * distance;

    if discriminant < 0 {
        return;
    }

    let sqrt_discriminant = (discriminant as f64).sqrt() as i64;
    let t1 = ((time - sqrt_discriminant) / 2).max(0);
    let t2 = ((time + sqrt_discriminant) / 2).min(time);

    if t1 > t2 {
        return;
    }

    let ans = t2 - t1; 
    println!("answer: {}", ans);
}
