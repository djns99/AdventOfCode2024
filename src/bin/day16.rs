use std::collections::{BinaryHeap, BTreeSet, HashMap, HashSet, VecDeque};
use std::{io, mem};
use std::mem::swap;

fn advance(pos: (usize, usize), dir: (isize, isize)) -> (usize, usize) {
    let newpos = (pos.0 as isize + dir.0, pos.1 as isize + dir.1);
    (newpos.0 as usize, newpos.1 as usize)
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Position {
    pos: (usize, usize),
    dir: (isize, isize),
}

fn part1(mut grid: Vec<Vec<char>>) {
    let mut seen = HashMap::new();
    let mut heap = BinaryHeap::new();
    let w = grid[0].len();
    let h = grid.len();

    heap.push((0 as i64, Position { pos: (1, h - 2), dir: (1, 0) }));
    let mut shortest_distance = w - 2 + h - 2;
    let mut items_processed = 0;
    while !heap.is_empty() {
        items_processed += 1;
        let (score, pos) = heap.pop().unwrap();
        if grid[pos.pos.1][pos.pos.0] == '#' {
            continue;
        }
        let distance = (w - 2 - pos.pos.0) + (pos.pos.1 - 1);
        if distance < shortest_distance {
            println!("New closest item {:?} {}", pos, distance);
            println!("Items processed {} heap size {}", items_processed, heap.len());
            shortest_distance = distance;
        }
        // println!("Processing {} {:?}", score, pos);
        if pos.pos == (w - 2, 1) {
            println!("Min score {}", score);
            return;
        }
        if *seen.entry(pos).or_insert(score) < score {
            continue;
        }

        let mut advance_dir = |newdir, newscore| {
            let newpos = advance(pos.pos, newdir);
            let fullpos = Position { pos: newpos, dir: newdir };
            if grid[newpos.1][newpos.0] != '#' && !seen.contains_key(&fullpos) {
                heap.push((score - newscore, fullpos));
            }
        };
        advance_dir(pos.dir, 1);
        advance_dir((pos.dir.1, pos.dir.0), 1001);
        advance_dir((-pos.dir.1, -pos.dir.0), 1001);
    }
    println!("Ooops");
}


fn part2(grid: Vec<Vec<char>>) {
    let mut tiles_on_path = HashSet::<(usize, usize)>::new();
    let mut seen = HashMap::<Position, (i64, Vec<Position>)>::new();
    let mut heap = BinaryHeap::new();
    let w = grid[0].len();
    let h = grid.len();

    heap.push((0 as i64, Position { pos: (1, h - 2), dir: (1, 0) }, Position { pos: (1, h - 2), dir: (1, 0) }));
    let mut shortest_distance = w - 2 + h - 2;
    let mut items_processed = 0;
    let mut bestscore = 0;

    while !heap.is_empty() {
        items_processed += 1;
        let (score, pos, prev) = heap.pop().unwrap();
        if grid[pos.pos.1][pos.pos.0] == '#' {
            continue;
        }
        let distance = (w - 2 - pos.pos.0) + (pos.pos.1 - 1);
        if distance < shortest_distance {
            println!("New closest item {:?} {}", pos, distance);
            println!("Items processed {} heap size {}", items_processed, heap.len());
            shortest_distance = distance;
        }
        // println!("Processing {} {:?}", score, pos);
        if pos.pos == (w - 2, 1) {
            println!("Part 2 min score {}", score);
            bestscore = score;
            heap.push((score, pos, prev));
            break;
        }
        if seen.entry(pos).and_modify(|val| if val.0 == score { val.1.push(prev) }).or_insert((score, vec![prev])).0 < score {
            continue;
        }

        let mut advance_dir = |newdir, newscore| {
            let newpos = advance(pos.pos, newdir);
            let fullpos = Position { pos: newpos, dir: newdir };
            if grid[newpos.1][newpos.0] != '#' && !seen.contains_key(&fullpos) {
                heap.push((score - newscore, fullpos, pos));
            }
        };
        advance_dir(pos.dir, 1);
        advance_dir((pos.dir.1, pos.dir.0), 1001);
        advance_dir((-pos.dir.1, -pos.dir.0), 1001);
    }

    let mut to_path: VecDeque<_> = heap.into_iter().take_while(|(score, _, _)| *score == bestscore).map(|x| (x.1, vec![x.2])).collect();
    println!("Number of paths {}", to_path.len());
    while !to_path.is_empty() {
        let (curr, next) = to_path.pop_front().unwrap();
        tiles_on_path.insert(curr.pos);
        for n in next {
            to_path.push_back((n, mem::take(&mut seen.get_mut(&n).unwrap().1)));
        }
    }
    println!("Number of used tiles {}", tiles_on_path.len());
}

fn main() {
    let lines: Vec<Vec<char>> = io::stdin()
        .lines()
        .map(|line| line.expect("Could not read line").chars().map(|x| match x {
            'S' | 'E' => '.',
            v => v
        }).collect())
        .collect();
    lines.iter().for_each(|row| {
        println!("{:?}", row);
    });


    part1(lines.clone());
    part2(lines);
}
