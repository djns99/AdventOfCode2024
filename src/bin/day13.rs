#![feature(iter_array_chunks)]
use std::io;

#[derive(Debug)]
struct Machine {
    a: (i64, i64),
    b: (i64, i64),
    target: (i64, i64),
}

fn part1(machines: &Vec<Machine>) {
    let mut costs = vec![];
    for m in machines {
        // Solve linear equation
        // x_A * A + x_B * B = x_target
        // y_A * A + y_B * B = y_target
        // A = (x_target - x_B * B) / x_A
        // y_B * B = y_target - (y_A * (x_target - x_B * B) / x_A)
        // y_B * B = y_target - y_A * x_target / x_A + y_A * x_B * B / x_A
        // (y_B - y_A * x_B / x_A) * B = y_target - y_A * x_target / x_A
        // B = (y_target - y_A * x_target / x_A) / (y_B - y_A * x_B / x_A)

        let (x_A, y_A) = (m.a.0 as f64, m.a.1 as f64);
        let (x_B, y_B) = (m.b.0 as f64, m.b.1 as f64);
        let (x_target, y_target) = (m.target.0 as f64, m.target.1 as f64);

        assert_ne!(x_A / y_A, x_B / y_B);

        let B = (y_target - y_A * x_target / x_A) / (y_B - y_A * x_B / x_A);
        let A = (x_target - x_B * B) / x_A;
        let check = (x_A * A.round() + x_B * B.round() == x_target)
         && (y_A * A.round() + y_B * B.round() == y_target);

        if check {
            costs.push(A * 3.0 + B);
        }

        println!("Results for machine {:?} are {} {}", m, A, B);
    }
    let score = costs.iter().sum::<f64>();
    println!("Minimum cost {}", score);
}


fn part2(mut machines: Vec<Machine>) {
    for m in &mut machines {
        m.target = (m.target.0 + 10000000000000, m.target.1 + 10000000000000);
    }
    part1(&machines);
}

fn main() {
    let lines: Vec<Machine> = io::stdin()
        .lines()
        .map(|line| line.expect("Could not read line"))
        .filter(|line| !line.is_empty())
        .array_chunks::<3>()
        .map(|machine| {
            let split_to_tuple = |s: &String, split| { let v: Vec<i64> = s.split(split).skip(1).map(|x| x.split(',').next().unwrap().parse::<i64>().unwrap()).collect(); (v[0], v[1]) };
            let a = split_to_tuple(&machine[0], "+");
            let b = split_to_tuple(&machine[1], "+");
            let target = split_to_tuple(&machine[2], "=");
            Machine {a, b, target}
        })
        .collect();
    lines.iter().for_each(|row| {
        println!("{:?}", row);
    });

    part1(&lines);
    part2(lines);
}
