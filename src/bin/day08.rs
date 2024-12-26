use std::{env, fs::File, io::{self, BufRead}};
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

struct Node {
    name: String,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(name: String) -> Self {
        Node {
            name,
            left: None,
            right: None,
        }
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: <program> <filename>");
        std::process::exit(1);
    }

    let input_file = File::open(&args[1])?;
    let reader = io::BufReader::new(input_file);
    
    let mut lines = reader.lines().map(|line| line.unwrap());
    
    let directions = lines.next().unwrap().trim().to_string();
    
    lines.next(); 
    
    let mut node_map: HashMap<String, Rc<RefCell<Node>>> = HashMap::new();

    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        
        let parent_name = line[0..3].to_string();
        let left_child_name = line[7..10].to_string();
        let right_child_name = line[12..15].to_string();

        let parent_node = node_map.entry(parent_name.clone())
            .or_insert_with(|| Rc::new(RefCell::new(Node::new(parent_name.clone()))))
            .clone();

        if !left_child_name.is_empty() {
            let left_child_node = node_map.entry(left_child_name.clone())
                .or_insert_with(|| {
                    Rc::new(RefCell::new(Node::new(left_child_name.clone())))
                }).clone();
                
            parent_node.borrow_mut().left = Some(left_child_node);
        }

        if !right_child_name.is_empty() {
            let right_child_node = node_map.entry(right_child_name.clone())
                .or_insert_with(|| {
                    Rc::new(RefCell::new(Node::new(right_child_name.clone())))
                }).clone();
                
            parent_node.borrow_mut().right = Some(right_child_node);
        }
    }

    println!("-- PART 1 --");
    part1(&node_map, &directions);
    println!("-- PART 2 --");
    part2(&node_map, &directions);
    Ok(())
}

fn part1(nodes: &HashMap<String, Rc<RefCell<Node>>>, directions: &str) {
    let mut current_node = nodes.get("AAA").cloned();
    let mut sum = 0;

    loop {
        if let Some(node) = current_node {
            let node_ref = node.borrow(); 
            //println!("Current node: {}", node_ref.name);
            if node_ref.name == "ZZZ" {
                println!("sum: {}", sum);
                return;
            }

            let direction = directions.chars().nth(sum % directions.len()).unwrap();
            //println!("Direction: {}", direction);

            current_node = match direction {
                'R' => node_ref.right.clone(),
                'L' => node_ref.left.clone(), 
                _ => None,
            };
            sum += 1;
        } 
    }
}

fn lcm_of_vec(numbers: &[u64]) -> u64 {
    fn gcd(a: u64, b: u64) -> u64 {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }

    fn lcm(a: u64, b: u64) -> u64 {
        (a * b) / gcd(a, b)
    }

    numbers.iter().copied().reduce(lcm).unwrap_or(1)
}
fn part2(nodes: &HashMap<String, Rc<RefCell<Node>>>, directions: &str) {
    let mut current_nodes: Vec<Rc<RefCell<Node>>> = Vec::new();
    let mut sums: Vec<u64> = Vec::new();

    for node in nodes.values() {
        let node_ref = node.borrow();
        if node_ref.name.ends_with('A') {
            current_nodes.push(node.clone());
        }
    }

    for mut current_node in current_nodes {
        let mut sum = 0;
        loop {
            let node = current_node;
            let node_ref = node.borrow(); 
            //println!("Current node: {}", node_ref.name);
            if node_ref.name.ends_with('Z') {
                //println!("Current node: {}", node_ref.name);
                break;
            }

            let direction = directions.chars().nth(sum % directions.len()).unwrap();
            //println!("Direction: {}", direction);

            if direction == 'R' {
                current_node = node_ref.right.clone().unwrap();
            } else {
                current_node = node_ref.left.clone().unwrap();
            }
            sum += 1;
        } 
        sums.push(sum as u64);
    }
    println!("sum: {}", lcm_of_vec(&sums));
}
