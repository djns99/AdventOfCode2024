use std::collections::{HashMap, HashSet};
use std::io;
use std::ops::Deref;

#[derive(Debug, Default)]
struct RadixTree {
    terminal: bool,
    next: [Option<Box<RadixTree>>; 26],
}

impl RadixTree {
    fn get_next(&self, prefix: &str) -> &Option<Box<RadixTree>> {
        &self.next[prefix.chars().next().unwrap() as usize - 'a' as usize]
    }
    fn get_next_mut(&mut self, prefix: &str) -> &mut Option<Box<RadixTree>> {
        &mut self.next[prefix.chars().next().unwrap() as usize - 'a' as usize]
    }
    fn add(&mut self, prefix: &str) {
        if prefix.is_empty() {
            self.terminal = true;
        } else {
            self.get_next_mut(prefix)
                .get_or_insert_with(|| Box::new(RadixTree { terminal: false, next: Default::default() }))
                .add(&prefix[1..]);
        }
    }

    fn longest_full_match(&self, prefix: &str) -> Option<usize> {
        if !prefix.is_empty() {
            let next = self.get_next(prefix);
            if next.is_some() {
                let bestmatch = next.as_ref().unwrap().longest_full_match(&prefix[1..]);
                if bestmatch.is_some() {
                    return Some(bestmatch.unwrap() + 1);
                }
            }
        }

        if self.terminal { Some(0) } else { None }
    }
}


fn solve(towels: &RadixTree, pattern: &str) -> bool {
    if pattern.is_empty() {
        return true;
    }
    let mut longest = towels.longest_full_match(pattern);
    while longest.is_some() {
        if solve(towels, &pattern[longest.unwrap()..]) {
            return true;
        }
        longest = towels.longest_full_match(&pattern[0..longest.unwrap() - 1]);
    }
    false
}

fn part1(towels: &RadixTree, patterns: &Vec<&String>) {
    let matches = patterns.iter().filter(|p| solve(towels, p)).count();
    println!("Total matches {}", matches)
}

fn solve_count(towels: &RadixTree, pattern: &str, cache: &mut HashMap<usize, u64>) -> u64 {
    if pattern.is_empty() {
        return 1;
    }
    match cache.get(&pattern.len()) {
        None => {
            let mut longest = towels.longest_full_match(pattern);
            let mut count = 0;
            while longest.is_some() {
                count += solve_count(towels, &pattern[longest.unwrap()..], cache);
                longest = towels.longest_full_match(&pattern[0..longest.unwrap() - 1]);
            }
            cache.insert(pattern.len(), count);
            count
        }
        Some(v) => *v
    }
}


fn part2(towels: &RadixTree, patterns: &Vec<&String>) {
    let matches: u64 = patterns.iter().map(|p| solve_count(towels, p, &mut HashMap::new())).sum();
    println!("Total matches {}", matches)
}


fn main() {
    let lines: Vec<String> = io::stdin()
        .lines()
        .map(|line| line.expect("Could not read line"))
        .collect();
    // lines.iter().for_each(|row| {
    //     println!("{:?}", row);
    // });

    let mut tree: RadixTree = Default::default();
    let towels = lines[0].split(", ").for_each(|x| tree.add(x));
    let patterns = lines.iter().skip(2).collect();

    part1(&tree, &patterns);
    part2(&tree, &patterns);
}
