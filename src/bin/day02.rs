use std::{env, fs::File, io::{self, BufRead}};

fn main() -> io::Result<()>
{
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 
    {
        eprintln!("Usage: <program> <filename>");
        std::process::exit(1);
    }

    let input_file = File::open(&args[1])?;
    let reader = io::BufReader::new(input_file);
    
    let mut data: Vec<Vec<Vec<i32>>> = Vec::new();

    for line in reader.lines() 
    {
        let line = line?;
        let row = line;
        let round = parse_data(row);
        data.push(round);
    }

    println!("-- PART 1 --");
    part1(data.clone(), 12, 13, 14);
    println!("-- PART 2 --");
    part2(data.clone());
    Ok(())
}


fn parse_data(input: String) -> Vec<Vec<i32>> {
    let mut round_data: Vec<Vec<i32>> = Vec::new();

    let input = input.split_once(":")
        .map(|(_, rounds)| rounds.trim())  // Remove the "Game x: " part
        .unwrap_or_else(|| input.as_str());  // If no colon found, keep the original

    let rounds: Vec<&str> = input.split(';')
        .map(|s| s.trim()) // Trim each round to avoid leading/trailing spaces
        .filter(|s| !s.is_empty())  // Avoid processing empty rounds
        .collect();

    for round in rounds {
        let mut color_counts = vec![0, 0, 0]; // [red, green, blue] -- Correct color order

        let parts: Vec<&str> = round.split(',')
            .map(|s| s.trim()) // Trim spaces from each part
            .collect();

        for part in parts {
            let tokens: Vec<&str> = part.split_whitespace().collect();
            if tokens.len() == 2 {
                let count: i32 = match tokens[0].parse() {
                    Ok(c) => c,
                    Err(_) => {
                        eprintln!("Invalid count in input: {}", tokens[0]);
                        continue; 
                    }
                };
                let color = tokens[1];

                match color {
                    "red" => color_counts[0] += count,  
                    "green" => color_counts[1] += count, 
                    "blue" => color_counts[2] += count,  
                    _ => {
                        eprintln!("Unknown color: {}", color);
                    },
                }
            }
        }

        round_data.push(color_counts);
    }

    round_data
}


fn part1(data: Vec<Vec<Vec<i32>>>, red: i32, green: i32, blue: i32)
{
    let mut sum:i32 = 0;
    let mut i:i32 = 0;

    for game in data
    {
        i += 1;
        let mut possible = true;
        for round in game
        {
            if round[0] > red || round[1] > green || round[2] > blue
            {
                possible = false;
                break;
            }
        }
        if possible
        {
            sum += i;
        }
    }
    println!("sum: {}", sum);
}

fn part2(data: Vec<Vec<Vec<i32>>>)
{
    let mut sum:i64 = 0;

    for game in data
    {
        let mut red:i32 = 0;
        let mut green:i32 = 0;
        let mut blue:i32 = 0;
        for round in game
        {
            if red < round[0]
            {
                red = round[0];
            }
            if green < round[1]
            {
                green = round[1];
            }
            if blue < round[2]
            {
                blue = round[2];
            }

        }
        println!("{}, {}, {}", red, green, blue);
        sum += (red * green * blue) as i64;
    }
    println!("sum: {}", sum);
}
