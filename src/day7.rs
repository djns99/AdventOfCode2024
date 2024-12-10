use std::io;

fn test1(target: u64, coeff: &mut [u64]) -> bool {
    if coeff.len() == 1 {
        return target == coeff[0];
    }
    if coeff[0] > target {
        return false;
    }

    let second = coeff[1];
    coeff[1] = coeff[0] + second;
    if test1(target, &mut coeff[1..]) {
        return true;
    }

    coeff[1] = coeff[0] * second;
    if test1(target, &mut coeff[1..]) {
        return true;
    }
    coeff[1] = second;

    false
}

fn test2(target: u64, coeff: &mut [u64]) -> bool {
    if coeff.len() == 1 {
        return target == coeff[0];
    }
    if coeff[0] > target {
        return false;
    }

    let second = coeff[1];
    coeff[1] = coeff[0] + second;
    if test2(target, &mut coeff[1..]) {
        return true;
    }

    coeff[1] = coeff[0] * second;
    if test2(target, &mut coeff[1..]) {
        return true;
    }

    let mut mul10 = 10;
    while mul10 <= second {
        mul10 *= 10;
    }
    coeff[1] = coeff[0] * mul10 + second;
    if test2(target, &mut coeff[1..]) {
        return true;
    }
    coeff[1] = second;

    false
}

fn test_fn(lines: &Vec<String>, test_fn: fn(u64, &mut [u64]) -> bool) {
    let parse_int = |x: &str| x.parse::<u64>().unwrap();
    let mut count = 0;
    for equation in lines {
        let [left, right] = equation.split(':').take(2).collect::<Vec<&str>>().try_into().unwrap();
        let target = parse_int(left);
        let mut coeff: Vec<u64> = right.split_whitespace().map(parse_int).collect();

        if test_fn(target, &mut coeff) {
            println!("Equation: {}, curr result {}", equation, count);
            count += target;
        }
    }
    println!("Count {}", count);
}

fn part1(lines: &Vec<String>) {
    test_fn(lines, test1);
}


fn part2(lines: &Vec<String>) {
    test_fn(lines, test2);
}

fn main() {
    let lines: Vec<String> = io::stdin()
        .lines()
        .map(|line| line.expect("Could not read line"))
        .collect();
    lines.iter().for_each(|row| {
        println!("{:?}", row);
    });

    part1(&lines);
    part2(&lines);
}
