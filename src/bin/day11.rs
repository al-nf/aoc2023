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

fn part1(data: &[Vec<char>]) {
    let mut cosmos: Vec<Vec<char>> = Vec::new();
    let mut row_empty: Vec<bool> = Vec::new();
    let mut col_empty: Vec<bool> = Vec::new();

    // cosmic expansion: vertical
    for row in data {
        let mut has_galaxy = false;
        for ch in row {
            if *ch == '#' {has_galaxy = true;}
        }
        if has_galaxy {
            row_empty.push(false);
        } else {
            row_empty.push(true);
        }
    }

    for i in 0..data[0].len() {
        let mut has_galaxy = false;
        for 
    }
}

fn part2(data: &[Vec<char>]) {

}

