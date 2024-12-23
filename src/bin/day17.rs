use std::io;

fn run(mut a: i64, mut b: i64, mut c: i64, program: &Vec<i64>) {
    let mut pc = 0;
    let parse_combo = |combo: i64, a, b, c| match combo {
        4 => a,
        5 => b,
        6 => c,
        v => v
    };
    while pc < program.len() {
        let literal = program[pc + 1];
        let combo = parse_combo(literal, a, b, c);
        match program[pc] {
            0 => {
                a = a / (1 << combo);
            }
            1 => {
                b = b ^ literal;
            }
            2 => {
                b = combo % 8;
            }
            3 => {
                if a != 0 {
                    pc = literal as usize;
                    continue;
                }
            }
            4 => {
                b = b ^ c;
            }
            5 => {
                print!("{},", combo % 8);
            }
            6 => {
                b = a / (1 << combo);
            }
            7 => {
                c = a / (1 << combo);
            }
            _ => panic!()
        }
        pc += 2;
    }
}

fn part1(a: i64, b: i64, c: i64, program: &Vec<i64>) {
    run(a, b, c, program);
    println!();
}


fn solve(mut a: u64, mut amask: u64, pos: usize, program: &Vec<u64>) -> Option<u64> {
    // 2,4, 1,3, 7,5, 4,7, 0,3, 1,5, 5,5, 3,0
    // bst 4 - b = a % 8;
    // bxl 3 - b ^= 3;
    // cdv 5 - c = a / (1 << b) = a >> b
    // bxc 7 - b ^= c
    // adv 3 - a = a / (1 << 3) = a >> 3
    // bxl 5 - b ^= 5
    // out 5 - out b
    // jnz 0

    println!("Testing a {} mask {} pos {}", a, amask, pos);
    let chunk_pos = (pos * 3) as u64;
    if pos == program.len() {
        if (a >> chunk_pos) != 0 {
            return None;
        }
        println!("Found solution {} a {} mask {} pos {}", a & amask, a, amask, pos);

        return Some(a & amask);
    }
    let srca = a;
    let srcamask = amask;
    let target = program[pos] ^ 5;

    'outer: for i in 0..8 {
        a = srca;
        amask = srcamask;
        let mut check_or_set = |bit, abs_pos| -> bool {
            if abs_pos >= 64 {
                false
            } else if (amask >> abs_pos) & 0x1 == 1 {
                ((a >> abs_pos) & 0x1) == bit
            } else {
                a |= bit << abs_pos;
                amask |= 1_u64 << abs_pos;
                true
            }
        };
        for bitpos in 0..3 {
            let bit = (i >> bitpos) & 1;
            if !check_or_set(bit, chunk_pos + bitpos) {
                continue 'outer;
            }
        }

        let shift = i ^ 3;
        let shiftval = target ^ shift; // The value we need to store in shifted position to work

        for bitpos in 0..3 {
            let bit = (shiftval >> bitpos) & 1;
            if !check_or_set(bit, chunk_pos + bitpos + shift) {
                continue 'outer;
            }
        }

        // If we reach here we have a currently valid solution
        let sol = solve(a, amask, pos + 1, program);
        if sol.is_some() {
            return sol;
        }
    }

    println!("No valid solution found");
    None
}

fn part2(a: i64, b: i64, c: i64, program: &Vec<i64>) {
    let srcprogram = program;
    let program = program.into_iter().map(|&x| x as u64).collect();
    let solution = solve(0, 0, 0, &program);
    run(solution.unwrap() as i64, 0, 0, srcprogram);
}


fn main() {
    let lines: Vec<String> = io::stdin()
        .lines()
        .map(|line| line.expect("Could not read line"))
        .collect();
    lines.iter().for_each(|row| {
        println!("{:?}", row);
    });

    let a = lines[0].split_ascii_whitespace().last().unwrap().parse().unwrap();
    let b = lines[1].split_ascii_whitespace().last().unwrap().parse().unwrap();
    let c = lines[2].split_ascii_whitespace().last().unwrap().parse().unwrap();
    let program = lines[4].split([' ', ',']).skip(1).map(|x| x.parse().unwrap()).collect();

    part1(a, b, c, &program);
    part2(a, b, c, &program);
}
