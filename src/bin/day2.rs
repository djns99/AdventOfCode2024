use std::io;
use std::iter::zip;

fn abs_diff(a: i32, b: i32) -> i32 {
    let diff = a - b;
    if diff < 0 { -diff } else { diff }
}

fn part1() {
    let mut input = String::new();

    let stdin = io::stdin();
    let mut safe = 0;
    while stdin.read_line(&mut input).is_ok() && !input.is_empty() {
        let mut input_iter = input.trim().split_ascii_whitespace();
        let mut last = input_iter.next().unwrap().parse::<i32>().unwrap();
        let mut curr = input_iter.next().unwrap().parse::<i32>().unwrap();
        let decreasing = curr < last;
        safe += 1;
        loop {
            if decreasing != (curr < last) { safe -= 1; break; }
            let diff = abs_diff(curr, last);
            if 0 == diff || diff >= 4 { safe -= 1; break; }
            last = curr;
            let val = input_iter.next();
            if val.is_none() { break; }
            curr = val.unwrap().parse::<i32>().unwrap();
        }
        input.clear();

    }
    println!("{}", safe);
}


fn solve(seq: &Vec<i32>, skip: usize) -> bool {
    let mut decreasing: Option<bool> = None;
    let iter_seq = seq.iter().take(skip).chain(seq.iter().skip(skip+1));
    for (last, curr) in zip(iter_seq.clone(), iter_seq.skip(1)) {
        if decreasing.is_some() && decreasing.unwrap() != (curr < last) { return false; }
        decreasing = Some(curr < last);
        let diff = abs_diff(*curr, *last);
        if 0 == diff || diff >= 4 { return false; }
    }
    true
}

fn part2() {
        let mut input = String::new();

        let stdin = io::stdin();
        let mut safe = 0;
        while stdin.read_line(&mut input).is_ok() && !input.is_empty() {
            let seq = input.trim().split_ascii_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            println!("{:?}", seq);
            for i in 0..seq.len() {
                if solve(&seq, i) {
                    println!("Seq is safe if dropping {}", i);
                    safe += 1; break; }
            }
            input.clear();
        }
        println!("{}", safe);
}

fn main() {
//     part1();
    part2();
}
