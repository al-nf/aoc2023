use std::{env, fs::File, io::{self, BufRead}, collections::VecDeque};

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

const DIRS: [(i32, i32); 4] = [
    (-1, 0),
    (0, 1),
    (1, 0),
    (0, -1),
];

fn part1(data: &Vec<Vec<char>>) {
    let mut start: (usize, usize) = (0, 0);
    for i in 0..data.len() {
        for j in 0..data[0].len() {
            if data[i][j] == 'S' {start = (i, j)}
        }
    }
    
    let can_move = |from: char, to: char, direction: usize| -> bool {
        let condition1 = match direction {
            0 => from == '|' ||  from == 'L' ||  from == 'J' ||  from == '.' ||  from == 'S',
            1 => from == '-' ||  from == 'L' ||  from == 'F' ||  from == '.' ||  from == 'S',
            2 => from == '|' ||  from == '7' ||  from == 'F' ||  from == '.' ||  from == 'S',
            3 => from == '-' ||  from == 'J' ||  from == '7' ||  from == '.' ||  from == 'S',
            _ => false
        };
        let condition2 = match direction {
            0 => to == '|' || to == '7' || to == 'F' || to == '.' || to == 'S',
            1 => to == '-' || to == 'J' || to == '7' || to == '.' || to == 'S',
            2 => to == '|' || to == 'L' || to == 'J' || to == '.' || to == 'S',
            3 => to == '-' || to == 'L' || to == 'F' || to == '.' || to == 'S',
            _ => false
        };
        condition1 && condition2
    };
    let bfs = |grid: &Vec<Vec<char>>, start: (usize, usize)| {
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut visited = vec![vec![false; cols]; rows];
        let mut q = VecDeque::new();
        let mut max_steps = 0;
        let mut furthest_pt = start;

        q.push_back((start, 0));
        visited[start.0][start.1] = true;

        while let Some(((x, y), steps)) = q.pop_front() {
            if steps > max_steps {
                max_steps = steps;
                furthest_pt = (x, y);
            }

            for (direction, &(dx, dy)) in DIRS.iter().enumerate() {
                let nx = (x as i32 + dx) as usize;
                let ny = (y as i32 + dy) as usize;

                if nx < rows && ny < cols && !visited[nx][ny] {
                    if can_move(grid[x][y], grid[nx][ny], direction) {
                        visited[nx][ny] = true;
                        q.push_back(((nx, ny), steps + 1));
                    }
                }
            }
        }

        (furthest_pt, max_steps)
    };

    let ((s_x, s_y), steps) = bfs(data, start);
    println!("steps: {}", steps);
}
fn part2(data: &Vec<Vec<char>>) {
}

