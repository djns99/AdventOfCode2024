use std::collections::{HashMap, HashSet};
use std::io;


fn try_step(grid: &Vec<Vec<char>>, x: isize, y: isize, target: usize, sources: &HashSet<usize>, next: &mut HashMap<usize, HashSet<usize>>) {
    if x < 0 || y < 0 {
        return;
    }
    let x = x as usize;
    let y = y as usize;
    let w = grid[0].len();
    let h = grid.len();
    if  x >= w || y >= h {
        return;
    }
    let target = target.to_string().chars().next().unwrap();
    // println!("Comparing {} vs {}", grid[y][x], target);
    if grid[y][x] == target {
        // println!("Found step to {:?}, sources {:?}", (x, y), sources);
        let entry = next.entry(y * w + x).and_modify(|c| { c.extend(sources); }).or_insert_with(|| sources.clone());
        // println!("Curr map at {:?} is {:?}", (x,y), entry);
    }
}

fn part1(grid: &Vec<Vec<char>>) {
    let w = grid[0].len();
    let h = grid.len();
    let mut index_pos: Vec<HashMap<usize, HashSet<usize>>> = vec![Default::default(); 10];
    index_pos[0] = grid.iter().flatten().enumerate().filter(|(_, c)| **c == '0').map(|(x, _)| {
        let mut set = HashSet::new();
        set.insert(x);
        (x, set)
    }).collect();
    for i in 0..9 {
        // println!("Results {:?}", index_pos[i]);
        for (pos, count) in index_pos[i].clone() {
            let (x, y) = ((pos % w) as isize, (pos / w)  as isize);
            try_step(&grid, x - 1, y, i+1, &count, &mut index_pos[i + 1]);
            try_step(&grid, x + 1, y, i+1, &count, &mut index_pos[i + 1]);
            try_step(&grid, x, y - 1, i+1, &count, &mut index_pos[i + 1]);
            try_step(&grid, x, y + 1, i+1, &count, &mut index_pos[i + 1]);
        }
    }

    // println!("Results {:?}", index_pos[9]);
    println!("Num trails {}", index_pos[9].values().map(|x| x.len()).sum::<usize>())
}

fn try_step2(grid: &Vec<Vec<char>>, x: isize, y: isize, target: usize, count: usize, next: &mut HashMap<usize, usize>) {
    if x < 0 || y < 0 {
        return;
    }
    let x = x as usize;
    let y = y as usize;
    let w = grid[0].len();
    let h = grid.len();
    if  x >= w || y >= h {
        return;
    }
    let target = target.to_string().chars().next().unwrap();
    // println!("Comparing {} vs {}", grid[y][x], target);
    if grid[y][x] == target {
        // println!("Found step to {:?}, sources {:?}", (x, y), sources);
        let entry = next.entry(y * w + x).and_modify(|c| { *c += count; }).or_insert(count);
        // println!("Curr map at {:?} is {:?}", (x,y), entry);
    }
}

fn part2(grid: &Vec<Vec<char>>) {
    let w = grid[0].len();
    let h = grid.len();
    let mut index_pos: Vec<HashMap<usize, usize>> = vec![Default::default(); 10];
    index_pos[0] = grid.iter().flatten().enumerate().filter(|(_, c)| **c == '0').map(|(x, _)| {
        (x, 1)
    }).collect();
    for i in 0..9 {
        // println!("Results {:?}", index_pos[i]);
        for (pos, count) in index_pos[i].clone() {
            let (x, y) = ((pos % w) as isize, (pos / w)  as isize);
            try_step2(&grid, x - 1, y, i+1, count, &mut index_pos[i + 1]);
            try_step2(&grid, x + 1, y, i+1, count, &mut index_pos[i + 1]);
            try_step2(&grid, x, y - 1, i+1, count, &mut index_pos[i + 1]);
            try_step2(&grid, x, y + 1, i+1, count, &mut index_pos[i + 1]);
        }
    }

    // println!("Results {:?}", index_pos[9]);
    println!("Num trails {}", index_pos[9].values().sum::<usize>())
}

fn main() {
    let lines: Vec<Vec<char>> = io::stdin()
        .lines()
        .map(|line| line.expect("Could not read line").chars().collect())
        .collect();
    lines.iter().for_each(|row| {
        println!("{:?}", row);
    });


    part1(&lines);
    part2(&lines);
}
