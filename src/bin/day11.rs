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

    for row in data {
        let mut has_galaxy = false;
        for ch in row {
            if *ch == '#' {has_galaxy = true;}
        }
        row_empty.push(!has_galaxy)
    }

    for i in 0..data[0].len() {
        let mut has_galaxy = false;
        for j in 0..data.len() {
            if data[j][i] == '#' {
                has_galaxy = true;
                break;
            }
        }
        col_empty.push(!has_galaxy); 
    }
    for i in 0..data.len() {
        let mut new_row: Vec<char> = Vec::new();
        
        for j in 0..data[i].len() {
            new_row.push(data[i][j]);
            if col_empty[j] {
                new_row.push('.');
            }
        }
        
        cosmos.push(new_row);
        
        if row_empty[i] {
            let mut empty_row = Vec::new();
            for j in 0..data[i].len() {
                empty_row.push('.');
                if col_empty[j] {
                    empty_row.push('.');
                }
            }
            cosmos.push(empty_row);
        }
    }
    
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    for i in 0..cosmos.len() {
        for j in 0..cosmos[0].len() {
            galaxies.push((i, j));
        }
    }
    let dist = |p1: (usize, usize), p2: (usize, usize)| -> i32 {
        (p2.0 as i32 - p1.0 as i32).abs() + (p2.1 as i32 - p1.1 as i32).abs()
    };
    
    let mut sum: i32 = 0;
    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            let galaxy1 = galaxies[i];
            let galaxy2 = galaxies[j];

            sum += dist(galaxy1, galaxy2);
        }
    }
    println!("sum: {}", sum);
}

fn part2(data: &[Vec<char>]) {
    let mut cosmos: Vec<Vec<char>> = Vec::new();
    let mut row_empty: Vec<bool> = Vec::new();
    let mut col_empty: Vec<bool> = Vec::new();

    for row in data {
        let mut has_galaxy = false;
        for ch in row {
            if *ch == '#' {has_galaxy = true;}
        }
        row_empty.push(!has_galaxy)
    }

    for i in 0..data[0].len() {
        let mut has_galaxy = false;
        for j in 0..data.len() {
            if data[j][i] == '#' {
                has_galaxy = true;
                break;
            }
        }
        col_empty.push(!has_galaxy); 
    }
    for i in 0..data.len() {
        let mut new_row: Vec<char> = Vec::new();
        
        for j in 0..data[i].len() {
            new_row.push(data[i][j]);
            if col_empty[j] {
                new_row.push('.');
            }
        }
        
        cosmos.push(new_row);
        
        if row_empty[i] {
            let mut empty_row = Vec::new();
            for j in 0..data[i].len() {
                empty_row.push('.');
                if col_empty[j] {
                    empty_row.push('.');
                }
            }
            cosmos.push(empty_row);
        }
    }
    
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    for i in 0..cosmos.len() {
        for j in 0..cosmos[0].len() {
            galaxies.push((i, j));
        }
    }
    let dist = |p1: (usize, usize), p2: (usize, usize)| -> u64 {
        ((p2.0 as i32 - p1.0 as i32).abs() + (p2.1 as i32 - p1.1 as i32).abs() * (1000000-1)) as u64
    };
    
    let mut sum: u64 = 0;
    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            let galaxy1 = galaxies[i];
            let galaxy2 = galaxies[j];

            sum += dist(galaxy1, galaxy2);
        }
    }
    println!("sum: {}", sum);
}

