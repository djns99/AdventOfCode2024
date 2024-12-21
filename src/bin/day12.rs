use std::collections::{BTreeSet, HashMap, HashSet};
use std::io;


fn part1(grid: &Vec<Vec<char>>) {
    let w = grid[0].len() as isize;
    let h = grid.len() as isize;
    let mut waiting = BTreeSet::<(usize, usize)>::new();
    let mut processing = BTreeSet::<(usize, usize)>::new();
    let mut done = HashSet::<(usize, usize)>::new();
    waiting.insert((0,0));

    let mut cost = 0;
    while !waiting.is_empty() {
        let (x, y) = waiting.pop_first().unwrap();
        processing.clear();
        processing.insert((x, y));
        let target = grid[y][x];
        let mut area = 0;
        let mut perimeter = 0;
        while !processing.is_empty() {
            let (x, y) = processing.pop_first().unwrap();
            if !done.insert((x,y)) { continue; }
            area += 1;
            for (xdir, ydir) in [(-1, 0), (1, 0), (0, -1), (0, 1)].into_iter() {
                let (nx, ny) = (x as isize + xdir, y as isize + ydir);
                if nx < 0 || nx >= w || ny < 0 || ny >= h {
                    perimeter += 1; // Found an edge that needs a fence
                    continue;
                }
                let (nx, ny) = (nx as usize, ny as usize);
                if grid[ny][nx] != target {
                    waiting.insert((nx, ny));
                    perimeter += 1; // Found an edge that needs a fence
                } else {
                    processing.insert((nx, ny));
                }
            }
        }
        // println!("Size for {} ({}, {}) is {} * {}", target, x, y, area, perimeter);
        cost += area * perimeter;
    }

    println!("Cost is {}", cost);
}

fn check_neighbour(grid: &Vec<Vec<char>>, done: &HashMap::<(usize, usize), [bool; 4]>, dir: (isize, isize), pos: (usize, usize), side: usize) -> i32 {
    let (xdir, ydir) = (dir.1.abs() as usize, dir.0.abs() as usize);
    let (x, y) = pos;

    let mut matches = 0;
    if x >= xdir && y >= ydir && grid[y - ydir][x - xdir] == grid[y][x] {
        match done.get(&(x - xdir, y - ydir)) {
            Some(fences) => {
                if fences[side] { matches += 1; }
            },
            _ => {}
        }
    }

    if x + xdir < grid[0].len() && y + ydir < grid.len() && grid[y + ydir][x + xdir] == grid[y][x] {
        match done.get(&(x + xdir, y + ydir)) {
            Some(fences) => {
                if fences[side] { matches += 1; }
            },
            _ => {}
        }
    }

    match matches {
        2 => -1, // Fusing two sides into one
        1 => 0, // Already connected to side
        0 => 1, // New side
        _ => panic!("Impossible number of matches")
    }
}

fn part2(grid: &Vec<Vec<char>>) {
    let w = grid[0].len() as isize;
    let h = grid.len() as isize;
    let mut waiting = BTreeSet::<(usize, usize)>::new();
    let mut processing = BTreeSet::<(usize, usize)>::new();
    let mut done = HashMap::<(usize, usize), [bool; 4]>::new();
    waiting.insert((0,0));

    let mut cost = 0;
    while !waiting.is_empty() {
        let (x, y) = waiting.pop_first().unwrap();
        if done.contains_key(&(x,y)) { continue; }
        processing.clear();
        processing.insert((x, y));
        let target = grid[y][x];
        let mut area = 0;
        let mut sides = 0;
        while !processing.is_empty() {
            let (x, y) = processing.pop_first().unwrap();
            if done.contains_key(&(x,y)) {
                continue;
            }
            area += 1;
            let mut has_fence = [false, false, false, false];
            for (side, (xdir, ydir)) in [(-1, 0), (1, 0), (0, -1), (0, 1)].into_iter().enumerate() {
                let (nx, ny) = (x as isize + xdir, y as isize + ydir);
                if nx < 0 || nx >= w || ny < 0 || ny >= h {
                    has_fence[side] = true;
                    sides += check_neighbour(&grid, &done, (xdir, ydir), (x, y), side);
                    // println!("++ Added Side {} for {} ({}, {}) is {} * {}", side, target, x, y, area, sides);
                    continue;
                }
                let (nx, ny) = (nx as usize, ny as usize);
                has_fence[side] = grid[ny][nx] != target;
                if has_fence[side] {
                    waiting.insert((nx, ny));
                    sides += check_neighbour(&grid, &done, (xdir, ydir), (x, y), side);
                    // println!("+- Added Side {} for {} ({}, {}) is {} * {}", side, target, x, y, area, sides);
                } else {
                    processing.insert((nx, ny));
                }

            }
            done.insert((x, y), has_fence);
        }
        println!("Size for {} ({}, {}) is {} * {}", target, x, y, area, sides);
        cost += area * sides;
    }

    println!("Cost is {}", cost);
}

fn main() {
    let lines: Vec<Vec<char>> = io::stdin()
        .lines()
        .enumerate()
        .map(|(y, line) | line.expect("Could not read line").chars().collect())
        .collect();
    lines.iter().for_each(|row| {
        println!("{:?}", row);
    });


    part1(&lines);
    part2(&lines);
}
