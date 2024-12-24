use std::collections::{BinaryHeap, BTreeSet, HashMap, HashSet, VecDeque};
use std::{io, mem};
use std::mem::swap;
use crate::Solution::{Count, Score};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Position {
    pos: (usize, usize),
    skip_pos: Option<((usize, usize), (usize, usize))>,
}

enum Solution {
    Score(i64),
    Count(i64),
}

fn solve(grid: &Vec<Vec<char>>, start: Position, end: Position, cutoff: Option<i64>) -> Solution {
    let mut seen = HashMap::new();
    let mut heap = BinaryHeap::new();
    let w = grid[0].len() as isize;
    let h = grid.len() as isize;

    let advance = |pos: (usize, usize), dir: (isize, isize)| -> Option<(usize, usize)> {
        let newpos = (pos.0 as isize + dir.0, pos.1 as isize + dir.1);
        if (newpos.0 < 0 || newpos.0 >= w || newpos.1 < 0 || newpos.1 >= h) { None } else {
            Some((newpos.0 as usize, newpos.1 as usize))
        }
    };

    let mut numsols = 0;

    heap.push((0 as i64, start, start.pos));

    while !heap.is_empty() {
        let (score, pos, prev) = heap.pop().unwrap();
        if cutoff.is_some() && cutoff.unwrap() > score {
            break;
        }
        let needs_skip = grid[pos.pos.1][pos.pos.0] == '#';
        if needs_skip && pos.skip_pos.is_some() {
            continue;
        }

        // println!("Processing {} {:?}", score, pos);
        if pos.pos == end.pos {
            println!("Min score {}", score);
            match cutoff {
                Some(_) => { numsols += 1; }
                None => { return Score(score); }
            }
        }
        let nocheat = Position { pos: pos.pos, skip_pos: None };
        if seen.contains_key(&nocheat) || *seen.entry(pos).or_insert(score) < score {
            continue;
        }

        let mut advance_dir = |newdir, newscore| {
            let Some(newpos) = advance(pos.pos, newdir) else { return; };
            if newpos == prev { return; }
            let skip_pos = if needs_skip { Some((prev, newpos)) } else { pos.skip_pos };
            let fullpos = Position { pos: newpos, skip_pos };
            let nocheat = Position { pos: newpos, skip_pos: None };
            if (grid[newpos.1][newpos.0] != '#' || skip_pos.is_none()) && !seen.contains_key(&fullpos) && !seen.contains_key(&nocheat) {
                heap.push((score - newscore, fullpos, pos.pos));
            }
        };
        advance_dir((0, 1), 1);
        advance_dir((0, -1), 1);
        advance_dir((1, 0), 1);
        advance_dir((-1, 0), 1);
    }
    Count(numsols)
}

fn part1(grid: Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) {
    let minsave = 100;
    let Score(bestscore) = solve(&grid, Position { pos: start, skip_pos: Some(Default::default()) }, Position { pos: end, skip_pos: None }, None) else { panic!() };
    let Count(numskips) = solve(&grid, Position { pos: start, skip_pos: None }, Position { pos: end, skip_pos: None }, Some(bestscore + minsave)) else { panic!() };
    println!("Number of improvements {}", numskips);
}


fn part2(grid: Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) {}

fn main() {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let lines: Vec<Vec<char>> = io::stdin()
        .lines().enumerate()
        .map(|(y, line)| line.expect("Could not read line").chars().enumerate().map(|(x, c)| match c {
            'S' => {
                start = (x, y);
                '.'
            }
            'E' => {
                end = (x, y);
                '.'
            }
            v => v
        }).collect())
        .collect();
    lines.iter().for_each(|row| {
        println!("{:?}", row);
    });


    part1(lines.clone(), start, end);
    part2(lines, start, end);
}
