use std::{env, fs::File, io::{self, BufRead}};
use aho_corasick::AhoCorasick;

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
    
    let mut data: Vec<String> = Vec::new();

    for line in reader.lines() 
    {
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

fn part1(data: &Vec<String>)
{
    let mut sum: i32 = 0;

    let mut ch1 = '-';
    let mut ch2 = '-';
    for i in 0..data.len()
    {
        for c in data[i].chars()
        {
            if c.is_numeric()
            {
                ch1 = c;
                break;
            }
        }

        for c in data[i].chars().rev()
        {
            if c.is_numeric()
            {
                ch2 = c;
                break;
            }
        }

        if ch1 != '-' && ch2 != '-'
        {
            let str = format!("{}{}", ch1, ch2);
            let term:i32 = str.parse().expect("Failed to parse string to int32");
            sum += term;

        }
    }
    println!("sum: {}", sum);
}

fn part2(data: &Vec<String>)
{
    let mut sum: i32 = 0;

    let mut strs: Vec<String> = Vec::with_capacity(data.len());
    for str in data
    {
        let patterns = &["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
        let str_tmp = str;
        let replace_with = &["1", "2", "3", "4", "5", "6", "7", "8", "9"];
        let ac = AhoCorasick::new(patterns);
        let result = ac.replace_all(str_tmp, replace_with);
        strs.push(str_tmp.to_string());
    }
    
    let mut ch1 = '-';
    let mut ch2 = '-';
    for i in 0..strs.len()
    {
        for c in strs[i].chars()
        {
            if c.is_numeric()
            {
                ch1 = c;
                break;
            }
        }

        for c in strs[i].chars().rev()
        {
            if c.is_numeric()
            {
                ch2 = c;
                break;
            }
        }

        if ch1 != '-' && ch2 != '-'
        {
            let str = format!("{}{}", ch1, ch2);
            let term:i32 = str.parse().expect("Failed to parse string to int32");
            sum += term;

        }
    }
    println!("sum: {}", sum);
}
