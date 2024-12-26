use std::collections::{HashMap, HashSet};
use std::io;
use std::ops::Deref;

fn sub_num_pad(src: char, dest: char) -> String {
    // +---+---+---+
    // | 7 | 8 | 9 |
    // +---+---+---+
    // | 4 | 5 | 6 |
    // +---+---+---+
    // | 1 | 2 | 3 |
    // +---+---+---+
    //     | 0 | A |
    //     +---+---+
    [
        ("0", [
        ("0", "A"),
        ("1", "^<A"),
        ("2", "^A"),
        ("3", "^>A"),
        ("4", "^^<A"),
        ("5", "^^A"),
        ("6", "^^>A"),
        ("7", "^^^<A"),
        ("8", "^^^A"),
        ("9", "^^^>A"),
        ("A", ">A"),
        ]),
    ("1", [
        ("0", ">v>A"),
        ("1", "A"),
        ("2", ">A"),
        ("3", ">>A"),
        ("4", "^A"),
        ("5", ">^A"),
        ("6", ">^>A"),
        ("7", "^^A"),
        ("8", ">^^A"),
        ("9", ">^^>A"),
        ("A", "v>A"),
        ]),
    ("2", [
        ("0", "vA"),
        ("1", "<A"),
        ("2", "A"),
        ("3", ">A"),
        ("4", "^<A"),
        ("5", "^A"),
        ("6", "^>A"),
        ("7", "^^<A"),
        ("8", "^^A"),
        ("9", "^^>A"),
        ("A", "v>A"),
        ]),
    ("3", [
        ("0", "<vA"),
        ("1", "<<A"),
        ("2", "<A"),
        ("3", "A"),
        ("4", "<^<A"),
        ("5", "<^A"),
        ("6", "^A"),
        ("7", "<^^<A"),
        ("8", "<^^A"),
        ("9", "^^A"),
        ("A", "vA"),
        ]),
    ("4", [
            ("0", ">vvA"),
            ("1", "vA"),
            ("2", ">vA"),
            ("3", ">v>A"),
            ("4", "A"),
            ("5", ">A"),
            ("6", ">>A"),
            ("7", "^A"),
            ("8", ">^A"),
            ("9", ">^>A"),
            ("A", ">vv>A"),
        ]),
    ("5", [
            ("0", "vvA"),
            ("1", "v<A"),
            ("2", "vA"),
            ("3", "v>A"),
            ("4", "<A"),
            ("5", "A"),
            ("6", ">A"),
            ("7", "^<A"),
            ("8", "^A"),
            ("9", "^>A"),
            ("A", "vv>A"),
        ]),
        ("6", [
        ("0", "v<vA"),
        ("1", "v<<A"),
        ("2", "v<A"),
        ("3", "vA"),
        ("4", "<<A"),
        ("5", "<A"),
        ("6", "A"),
        ("7", "<^<A"),
        ("8", "<^A"),
        ("9", "^A"),
        ("A", "vvA"),
        ]),
    ("9", [
        ("0", "vv<vA"),
        ("1", "vv<<A"),
        ("2", "vv<A"),
        ("3", "vvA"),
        ("4", "v<<A"),
        ("5", "v<A"),
        ("6", "vA"),
        ("7", "<<A"),
        ("8", "<A"),
        ("9", "A"),
        ("A", "vvvA"),
        ]),

    ];
    String::new()
}

fn part1(towels: &RadixTree, patterns: &Vec<&String>) {
    let matches = patterns.iter().filter(|p| solve(towels, p)).count();
    println!("Total matches {}", matches)
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
