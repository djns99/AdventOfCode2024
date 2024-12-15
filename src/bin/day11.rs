use std::collections::{HashMap, HashSet};
use std::io;
use std::mem::swap;
use crate::AdvanceResult::{Replace, Split};

fn digit_count(input: usize) -> u32 {
    if input == 0 {
        return 1;
    }
    input.ilog10() + 1
}

fn split(input: usize, digits: u32) -> (usize, usize) {
    let split = usize::pow(10, digits / 2);
    (input / split, input % split)
}

enum AdvanceResult {
    Split((usize, usize)),
    Replace(usize)
}

fn advance(v: usize) -> Vec<usize> {
    if v == 0 {
        return vec![1];
    }
    let ds = digit_count(v);
    if ds % 2 == 0 {
        let s = split(v, ds);
        return vec![s.0, s.1];
    } else {
        return vec![v * 2024];
    }
}

fn part1(mut grid: Vec<usize>) {
    let mut grid_other = Vec::<usize>::new();
    for _ in 0..25 {
        grid_other.clear();
        for &v in &grid {
            for nv in advance(v) {
                grid_other.push(nv);
            };
        }

        swap(&mut grid, &mut grid_other);
        if grid.len() < 30 {
            println!("Grid {:?}", grid);
        }
        println!("Result {}", grid.len());
    }

}

fn solve_stone(v: usize, depth: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    if depth == 75 {
        return 1;
    }
    match cache.get(&(v, depth)) {
        Some(count) => { *count }
        None => { let result = advance(v).iter().map(|&x| solve_stone(x, depth + 1, cache)).sum::<usize>(); cache.insert((v, depth), result); result }
    }
}

fn part2(grid: Vec<usize>) {
    let mut cache = HashMap::<(usize, usize), usize>::new();
    let result: usize = grid.into_iter().map(|v| solve_stone(v, 0, &mut cache)).sum();
    println!("Result is {}", result);
}

fn main() {
    let lines: Vec<usize> = io::stdin()
        .lines()
        .next().unwrap().unwrap()
        .split_ascii_whitespace()
        .map(|line| line.parse::<usize>().unwrap())
        .collect();
    println!("Input: {:?}", lines);


    part1(lines.clone());
    part2(lines);
}
