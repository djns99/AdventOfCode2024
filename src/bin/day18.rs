#![feature(map_try_insert)]

use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::io;
use std::iter::zip;

fn solve(grid: &Vec<Vec<bool>>, start: (usize, usize), end: (usize, usize), path: &[(usize, usize)]) -> Option<(i32, HashMap<(usize, usize), (usize, usize)>)> {
    type Deque = VecDeque::<(i32, (usize, usize), (usize, usize))>;
    let mut queue = Deque::new();
    let mut seen = HashMap::<(usize, usize), (usize, usize)>::new();

    queue.push_back((0, start, start));
    let mut push_neighbours = |queue: &mut Deque, steps, curr| {
        let (x, y) = curr;
        if x > 0 {
            queue.push_back((steps, (x - 1, y), curr));
        }
        if x < grid[0].len() -1 {
            queue.push_back((steps, (x + 1, y), curr));
        }
        if y > 0 {
            queue.push_back((steps, (x, y - 1), curr));
        }
        if y < grid.len() - 1 {
            queue.push_back((steps, (x, y + 1), curr));
        }
    };

    // Skips the first item in the path, which is pushed above
    for (prev, curr) in zip(path.iter(), path.iter().skip(1)) {
        seen.insert(*curr, *prev);
        push_neighbours(&mut queue, 0, *curr);
    }

    while !queue.is_empty() {
        let (steps, curr, prev) = queue.pop_front().unwrap();
        let (x, y) = curr;
        if seen.try_insert(curr, prev).is_err() || grid[y][x] {
            continue;
        }
        if curr == end {
            return Some((steps, seen));
        }

        push_neighbours(&mut queue,steps + 1, curr);
    }

    None
}

fn part1(blocks: &Vec<(usize, usize)>) {
    let w = 71;
    let time = 1024;
    let mut grid = vec![vec![false; w]; w];
    for &(x, y) in blocks.into_iter().take(time) {
        grid[y][x] = true;
    }

    grid.iter().for_each(|x| {
        println!("{}", x.iter().map(|&v| if v { '#' } else { '.' }).collect::<String>());
    });

    println!("Part 1 solution {}", solve(&grid, (0,0), (w-1, w-1), &[]).unwrap().0);
}

fn part2(blocks: &Vec<(usize, usize)>) {
    // let w = 7;
    // let time = 12;
    let w = 71;
    let time = 1024;
    let mut grid = vec![vec![false; w]; w];
    for &(x, y) in blocks.into_iter().take(time) {
        grid[y][x] = true;
    }



    let mut path = VecDeque::new();
    let mut lookup = None;

    for  &(x, y) in blocks.into_iter().skip(time) {
        grid[y][x] = true;

        let get_path = |grid, path: &mut VecDeque<(usize, usize)>| {
            let start = (0, 0);
            let mut end = (w-1, w-1);
            let (_, map) = solve(grid, (0,0), end, path.make_contiguous())?;
            path.clear();
            while end != start{
                path.push_front(end);
                end = *map.get(&end)?;
            }
            path.push_front(start);
            let map: HashMap<(usize, usize), usize> = path.iter().enumerate().map(|(x,y)| (*y, x)).collect();
            Some(map)
        };

        if lookup.is_none() {
            lookup = get_path(&grid, &mut path);
        }

        let index = lookup.as_ref().unwrap().get(&(x, y));
        if index.is_some() {
            path.truncate(*index.unwrap());
            lookup = get_path(&grid, &mut path);
            if lookup.is_none() {
                println!("Block {},{} was last block", x, y);
                break;
            }
        }
    }


}


fn main() {
    let lines: Vec<(usize, usize)> = io::stdin()
        .lines()
        .map(|line| line.expect("Could not read line"))
        .map(|line| {
            let mut it = line.split(',').map(|x| x.parse().unwrap());
            (it.next().unwrap(), it.next().unwrap())
        })
        .collect();
    // lines.iter().for_each(|row| {
    //     println!("{:?}", row);
    // });

    part1(&lines);
    part2(&lines);
}
