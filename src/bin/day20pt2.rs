use std::collections::{BinaryHeap, BTreeSet, HashMap, HashSet, VecDeque};
use std::{io, mem};
use std::mem::swap;
use crate::Solution::{Costs, Count, Score};

// #[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
// struct Position {
//     pos: usize,
//     skip_pos: Option<(usize, usize)>,
// }

enum Solution {
    Score(i64),
    Costs(HashMap<usize, i64>),
    Count(i64),
}

struct GridEntry {
    neighbours: Vec<(usize, i64)>,
}

fn make_grid_graph(grid: &Vec<Vec<char>>, cheatsize: isize) -> HashMap<usize, GridEntry> {
    let w = grid[0].len() as isize;
    let h = grid.len() as isize;
    let is_valid_step = |pos: (usize, usize), dir: (isize, isize), neighbours: &mut Vec<(usize, i64)>| {
        let (x, y) = (pos.0 as isize + dir.0, pos.1 as isize + dir.1);
        if x < 0 || x >= w || y < 0 || y >= h || grid[y as usize][x as usize] == '#' {
            false
        } else {
            neighbours.push(((y * w + x) as usize, (dir.0.abs() + dir.1.abs()) as i64));
            true
        }
    };
    grid.iter().enumerate().map(|(y, row)| {
        row.iter().enumerate().filter_map(move |(x, &c)| {
            if c == '#' { return None; }

            let pos = (x, y);
            let key = y * (w as usize) + x;
            let mut neighbours = vec![];
            // let has_up = is_valid_step(pos, (0, -1), &mut neighbours, 1);
            // let has_down = is_valid_step(pos, (0, 1), &mut neighbours, 1);
            // let has_left = is_valid_step(pos, (-1, 0), &mut neighbours, 1);
            // let has_right = is_valid_step(pos, (1, 0), &mut neighbours, 1);

            // let mut cheats = vec![];
            // let cheatsize = 20_isize;
            for ydir in -cheatsize..(cheatsize + 1) {
                for xdir in -(cheatsize - ydir.abs())..(cheatsize + 1 - ydir.abs()) {
                    if ydir == 0 && xdir == 0 { continue; }
                    // let left = has_left || xdir >= 0;
                    // let right = has_right || xdir <= 0;
                    // let up = has_up || ydir >= 0;
                    // let down = has_down || ydir <= 0;
                    // if !left && !right && !up && !down {
                    //     // We add a cheat if we can't trivially step closer to this node direction
                    //     is_valid_step(pos, (xdir, ydir), &mut cheats);
                    // }
                    is_valid_step(pos, (xdir, ydir), &mut neighbours);
                }
            }
            // let neighbours = neighbours.into_iter().map(|v| (v, false)).chain(
            //     cheats.into_iter().map(|v| (v, true))
            // ).collect();


            let entry = GridEntry { neighbours };
            Some((key, entry))
        })
    }).flatten().collect()
}

fn solve(grid: &Vec<Vec<char>>, start: (usize, usize), end: (usize, usize), cutoff: Option<i64>, finish_cache: Option<HashMap<usize, i64>>) -> Solution {
    let cheatsize = if cutoff.is_some() { 20_isize } else { 1_isize };
    let graph = make_grid_graph(&grid, cheatsize);
    println!("Built graph cheatsize {}", cheatsize);
    let start = start.0 + start.1 * grid[0].len();
    let end = end.0 + end.1 * grid[0].len();
    let mut seen = HashMap::new();
    let mut heap = BinaryHeap::new();
    let w = grid[0].len() as isize;
    let h = grid.len() as isize;

    // let advance = |pos: (usize, usize), dir: (isize, isize)| -> Option<(usize, usize)> {
    //     let newpos = (pos.0 as isize + dir.0, pos.1 as isize + dir.1);
    //     if (newpos.0 < 0 || newpos.0 >= w || newpos.1 < 0 || newpos.1 >= h) { None } else {
    //         Some((newpos.0 as usize, newpos.1 as usize))
    //     }
    // };

    let mut numsols = 0;

    heap.push((0 as i64, start));

    while !heap.is_empty() {
        let (score, pos) = heap.pop().unwrap();
        if cutoff.is_some() && cutoff.unwrap() > score {
            println!("Reached cutoff");
            break;
        }

        println!("Processing {} {:?}", score, pos);
        if pos == end {
            println!("Min score {}", score);
            assert!(cutoff.is_none());
            return Score(score);
            // match cutoff {
            //     Some(_) => { numsols += 1; }
            //     None => { return Score(score); }
            // }
        }
        if *seen.entry(pos).or_insert(score) != score {
            continue;
        }

        let neighbours = &graph.get(&pos).unwrap().neighbours;
        // println!("Neighbours {}", neighbours.len());
        for &(newpos, cost) in neighbours {
            if cost > 1 {
                assert!(finish_cache.is_some());
                let finalcost = score - cost + finish_cache.as_ref().unwrap().get(&newpos).unwrap();
                if finalcost >= cutoff.unwrap() {
                    println!("Found valid cheat {}", finalcost);
                    numsols += 1;
                }
                continue;
            }
            if seen.contains_key(&newpos) { continue; }
            heap.push((score - 1, newpos));
        }
        // let mut advance_dir = |newdir, newscore| {
        //     let Some(newpos) = advance(pos.pos, newdir) else { return; };
        //     if newpos == prev { return; }
        //     let skip_pos = if needs_skip { Some((prev, newpos)) } else { pos.skip_pos };
        //     let fullpos = Position { pos: newpos, skip_pos };
        //     let nocheat = Position { pos: newpos, skip_pos: None };
        //     if (grid[newpos.1][newpos.0] != '#' || skip_pos.is_none()) && !seen.contains_key(&fullpos) && !seen.contains_key(&nocheat) {
        //         heap.push((score - newscore, fullpos, pos.pos));
        //     }
        // };
        // advance_dir((0, 1), 1);
        // advance_dir((0, -1), 1);
        // advance_dir((1, 0), 1);
        // advance_dir((-1, 0), 1);
    }
    if end as isize >= w * h {
        Costs(seen)
    } else {
        Count(numsols)
    }
}

fn part1(grid: Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) {}


fn part2(grid: Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) {
    let minsave = 100;
    let Score(bestscore) = solve(&grid, start, end, None, None) else { panic!() };
    let Costs(costs) = solve(&grid, end, (grid[0].len(), grid.len()), None, None) else { panic!() };
    let Count(numskips) = solve(&grid, start, end, Some(bestscore + minsave), Some(costs)) else { panic!() };
    println!("Number of improvements {}", numskips);
}

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
