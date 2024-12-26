use std::{env, fs::File, io::{self, BufRead}};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: <program> <filename>");
        std::process::exit(1);
    }

    let input_file = File::open(&args[1])?;
    let reader = io::BufReader::new(input_file);

    let mut data: Vec<Vec<i32>> = Vec::new(); 

    for line in reader.lines() {
        let line = line?; 
        let numbers: Vec<i32> = line
            .split_whitespace() 
            .filter_map(|s| s.parse::<i32>().ok()) 
            .collect();
        
        data.push(numbers); 
    }

    println!("-- PART 1 --");
    part1(&data);
    println!("-- PART 2 --");
    part2(&data);
    Ok(())
}

fn part1(data: &Vec<Vec<i32>>) {
    let generate_history = |sequence: Vec<i32>| -> Vec<i32>{
        let mut result: Vec<i32> = Vec::new();
        for i in 1..sequence.len() {
            result.push(sequence[i] - sequence[i-1]);
        }
        //println!("{:?}", result);
        result
    };
    let all_constant = |sequence: &Vec<i32>| -> bool {
        let a = sequence[0];
        for &num in sequence {
            if num != a { return false; }
        }
        return true;
    };
    let solve = |sequence: Vec<i32>| -> i32 {
        let mut histories: Vec<Vec<i32>> = Vec::new();
        let mut sum = sequence[sequence.len()-1];
        histories.push(sequence.clone());

        while !all_constant(&histories[histories.len()-1].clone()) {
            histories.push(generate_history(histories[histories.len()-1].clone()));
        }

        for i in 1..histories.len() {
            sum += histories[i][histories[i].len()-1]
        }
        sum
    };
    let mut sum = 0;
    for nums in data {
        sum += solve(nums.to_vec());
    }
    println!("sum: {}", sum);
}

fn part2(data: &Vec<Vec<i32>>) {
    let generate_history = |sequence: Vec<i32>| -> Vec<i32>{
        let mut result: Vec<i32> = Vec::new();
        for i in 1..sequence.len() {
            result.push(sequence[i] - sequence[i-1]);
        }
        //println!("{:?}", result);
        result
    };
    let all_constant = |sequence: &Vec<i32>| -> bool {
        let a = sequence[0];
        for &num in sequence {
            if num != a { return false; }
        }
        return true;
    };
    let solve = |sequence: Vec<i32>| -> i32 {
        let mut histories: Vec<Vec<i32>> = Vec::new();
        let mut sum = sequence[sequence.len()-1];
        histories.push(sequence.clone());

        while !all_constant(&histories[histories.len()-1].clone()) {
            histories.push(generate_history(histories[histories.len()-1].clone()));
        }

        for i in 1..histories.len() {
            sum += histories[i][histories[i].len()-1]
        }
        sum
    };
    let mut sum = 0;
    for nums in data {
        let tmp: Vec<i32> = nums.iter().rev().cloned().collect();
        sum += solve(tmp.to_vec());
    }
    println!("sum: {}", sum);
}

