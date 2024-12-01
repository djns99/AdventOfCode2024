use std::collections::{hash_map, HashMap};
use std::io;
use std::iter::{Map, zip};

fn part1() {
    println!("Hello, world!");

    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();
    let mut input = String::new();

    let stdin = io::stdin();
    while stdin.read_line(&mut input).is_ok() && !input.is_empty() {
        let mut input_iter = input.trim().split_ascii_whitespace();
        vec1.push(input_iter.next().unwrap().to_string());
        vec2.push(input_iter.next().unwrap().to_string());
        println!("{} {}", vec1.last().unwrap(), vec2.last().unwrap());
        input.clear();
    }
    vec1.sort();
    vec2.sort();
    let mut sum = 0;
    for (v1, v2) in zip(vec1, vec2) {
        let val = v1.parse::<i32>().unwrap() - v2.parse::<i32>().unwrap();
        sum += if val > 0 { val } else { -val }
    }
    println!("Value {}", sum);
}

fn part2() {
    let mut map = HashMap::new();
    let mut seq = Vec::new();
    let mut input = String::new();

    let stdin = io::stdin();
    while stdin.read_line(&mut input).is_ok() && !input.is_empty() {
        let mut input_iter = input.trim().split_ascii_whitespace();
        let key = input_iter.next().unwrap().parse::<i32>().unwrap();
        let mul = input_iter.next().unwrap().parse::<i32>().unwrap();
        *map.entry(mul).or_insert(0) += 1;
        seq.push(key);
        input.clear();
    }
    let mut sum = 0;
    for v in seq {
        sum += map.get(&v).unwrap_or(&0) * v;
    }
    println!("Value {}", sum);
}

fn main() {
    // part1();
    part2();
}
