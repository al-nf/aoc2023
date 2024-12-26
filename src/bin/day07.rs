use std::{env, fs::File, io::{self, BufRead}, collections::HashMap};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: <program> <filename>");
        std::process::exit(1);
    }

    let input_file = File::open(&args[1])?;
    let reader = io::BufReader::new(input_file);
    
    let mut data: Vec<(String, i32)> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            let string_part = parts[0].to_string();
            let int_part: i32 = parts[1].parse().expect("Failed to parse integer");
            data.push((string_part, int_part));
        } else {
            eprintln!("Invalid line format: {}", line);
        }
    }

    println!("-- PART 1 --");
    part1(&data);
    println!("-- PART 2 --");
    part2(&data);
    Ok(())
}

fn convert_values(input: String, part2: bool) -> Vec<i32> {
    let mut output: Vec<i32> = Vec::new();
    for ch in input.chars() {
        if part2 {
            let part: i32 = match ch {
                'J' => 0,
                '2' => 1,
                '3' => 2,
                '4' => 3,
                '5' => 4,
                '6' => 5,
                '7' => 6,
                '8' => 7,
                '9' => 8,
                'T' => 9,
                'Q' => 10,
                'K' => 11,
                'A' => 12,
                _ => {
                    println!("error");
                    -1
                }
            };
            if part != -1 {
                output.push(part);
            }
        } else {
            let part: i32 = match ch {
                '2' => 0,
                '3' => 1,
                '4' => 2,
                '5' => 3,
                '6' => 4,
                '7' => 5,
                '8' => 6,
                '9' => 7,
                'T' => 8,
                'J' => 9,
                'Q' => 10,
                'K' => 11,
                'A' => 12,
                _ => {
                    println!("error");
                    -1
                }
            };
            if part != -1 {
                output.push(part);
            }
        }

    }

    /*
    if part2 {
        let mut frequency: HashMap<i32, i32> = HashMap::new();

        // Count the frequency of each number
        for &num in &output {
            *frequency.entry(num).or_insert(0) += 1;
        }

        // Find the most frequent and largest integer
        let mut most_frequent = output[0];
        let mut max_count = 0;

        for (&num, &count) in &frequency {
            if count > max_count || (count == max_count && num > most_frequent) {
                most_frequent = num;
                max_count = count;
            }
        }

        for num in &output {
            print!("{} ", num);
        }
        println!("");
        for i in 0..output.len() {
            if output[i] == 0 {
                output[i] = most_frequent;
            }
        }
        for num in &output {
            print!("{} ", num);
        }
        println!("");
        println!("");
    }
    */
    /* 
     * This is actually the correct way to evaluate a hand like in poker. I spent over half an hour
     * doing this just to find that the puzzle description does not want this.

    let mut frequencies: HashMap<i32, usize> = HashMap::new();
    for &value in &output {
        *frequencies.entry(value).or_insert(0) += 1;
    }

    let mut freq_pairs: Vec<(i32, usize)> = frequencies.iter()
        .map(|(&value, &count)| (value, count))
        .collect();

    freq_pairs.sort_unstable_by(|a, b| {
        let freq_cmp = b.1.cmp(&a.1);
        if freq_cmp == std::cmp::Ordering::Equal {
            return b.0.cmp(&a.0);
        }
        freq_cmp
    });

    output.clear(); 
    for (value, count) in freq_pairs {
        for _ in 0..count {
            output.push(value);
        }
    }
    */
    output
}

fn replace_jokers(hand: String) -> String {
    let mut frequencies: HashMap<char, usize> = HashMap::new();
    
    for ch in hand.chars() {
        if ch != 'J' {
            *frequencies.entry(ch).or_insert(0) += 1;
        }
    }

    let most_frequent_card = frequencies.iter()
        .max_by_key(|&(_, count)| count)
        .map(|(&value, _)| value)
        .unwrap_or('0');

    hand.chars().map(|ch| if ch == 'J' { most_frequent_card } else { ch }).collect()
}

fn classify_hand(hand: String, part2: bool) -> i32 {
    let mut counts = HashMap::new();
    
    if part2 {
        let new_hand = replace_jokers(hand);
        for card in new_hand.chars() {
            *counts.entry(card).or_insert(0) += 1;
        }
    }
    else {
        for card in hand.chars() {
            *counts.entry(card).or_insert(0) += 1;
        }
    }

    

    let mut count_values: Vec<_> = counts.values().cloned().collect();
    count_values.sort_unstable_by(|a, b| b.cmp(a)); 
    
    match count_values.as_slice() {
        [5] => 6,      
        [4, 1] => 5,    
        [3, 2] => 4,     
        [3, 1, 1] => 3,   
        [2, 2, 1] => 2,  
        [2, 1, 1, 1] => 1,
        _ if count_values.iter().all(|&count| count == 1) => 0,
        _ => -1,          
    }
}

fn part1(data: &[(String, i32)]) {
    let mut hands: Vec<(Vec<i32>, i32, i32)> = Vec::new();
    for (hand, bid) in data {
        let cards: Vec<i32> = convert_values(hand.to_string(), false);
        let value = classify_hand(hand.to_string(), false);
        hands.push((cards, value, *bid));
    }

    hands.sort_by(|a, b| {
        if a.1 != b.1 {
            return a.1.cmp(&b.1);
        }

        for i in 0..a.0.len() {
            if a.0[i] != b.0[i] {
                return a.0[i].cmp(&b.0[i]);
            }
        }
        
        std::cmp::Ordering::Equal
    });

    let mut sum: u64 = 0;
    for i in 0..hands.len() {
        /*
        println!("{}, {}, {}", hands[i].1, hands[i].2, i+1);
        for card in &hands[i].0 {
            print!("{} ", card);
        }
        println!("");
        println!("");
        */
        sum += hands[i].2 as u64 * (i+1) as u64;
    }

    println!("sum: {}", sum);
}
fn part2(data: &[(String, i32)]) {
    let mut hands: Vec<(Vec<i32>, i32, i32)> = Vec::new();
    for (hand, bid) in data {
        let cards: Vec<i32> = convert_values(hand.to_string(), true);
        let value = classify_hand(hand.to_string(), true);
        hands.push((cards, value, *bid));
    }

    hands.sort_by(|a, b| {
        if a.1 != b.1 {
            return a.1.cmp(&b.1);
        }

        for i in 0..a.0.len() {
            if a.0[i] != b.0[i] {
                return a.0[i].cmp(&b.0[i]);
            }
        }
        
        std::cmp::Ordering::Equal
    });

    let mut sum: u64 = 0;
    for i in 0..hands.len() {
        /*
        println!("{}, {}, {}", hands[i].1, hands[i].2, i+1);
        for card in &hands[i].0 {
            print!("{} ", card);
        }
        println!("");
        println!("");
        */
        sum += hands[i].2 as u64 * (i+1) as u64;
    }

    println!("sum: {}", sum);
}
