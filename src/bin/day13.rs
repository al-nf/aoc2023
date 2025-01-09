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
            let chars: Vec<char> = line.chars().collect();
            map.push(chars);
        } else {
            data.push(map.clone());
            map.clear();
        }
    }
    data.push(map.clone());
    map.clear();

    println!("-- PART 1 --");
    part1(&data);
    println!("-- PART 2 --");
    part2(&data);
    Ok(())
}

fn part1(data: &[Vec<Vec<char>>]) {
    let mut sum: u64 = 0;

    let columns_are_equal = |pattern: Vec<Vec<char>>, col1: usize, col2: usize| -> bool {
        for i in 0..pattern.len() {
            if pattern[i][col1] != pattern[i][col2] {
                return false;
            }
        }
        true
    };

    let perfect_reflection = |pattern: Vec<Vec<char>>, horizontal: bool, index: usize| -> bool {
        if horizontal {
            if index == 0 || index == pattern.len()-1 {
                return false;
            }
            let mut lower = index;
            let mut upper = index + 1;
            
            while upper < pattern.len() {
                if pattern[lower] != pattern[upper] {
                    return false;
                }
                if lower == 0 {
                    return false;
                }
                lower -= 1;
                upper += 1;
            }
            return true;
        }
        if index == 0 || index == pattern[0].len()-1 {
            return false;
        }

        let mut lower = index;
        let mut upper = index + 1;

        while upper < pattern[0].len() {
            if !columns_are_equal(pattern.clone(), lower, upper) {
                return false;
            }
            if lower == 0 {
                return false;
            }
            lower -= 1;
            upper += 1;
        }
        return true;
    };
    for pattern in data.iter() {
        for i in 0..pattern.len() {
            if perfect_reflection(pattern.to_vec(), true, i) {
                sum += 100 * (i+1) as u64;
            }
        }

        for i in 0..pattern[0].len() {
            if perfect_reflection(pattern.to_vec(), false, i) {
                sum += (i+1) as u64;
            }
        }
    }
    println!("sum: {}", sum);
}

fn part2(data: &[Vec<Vec<char>>]) {

}
