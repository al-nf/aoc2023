use std::{env, fs::File, io::{self, BufRead}};

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
        let chars: Vec<char> = line.chars().collect();
        data.push(chars); 
    }

    println!("-- PART 1 --");
    part1(&data);
    println!("-- PART 2 --");
    part2(&data);
    Ok(())
}

fn part1(data: &Vec<Vec<char>>) {
}

fn part2(data: &Vec<Vec<char>>) {
}

