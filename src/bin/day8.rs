use std::collections::HashMap;
use std::io;


fn part1(grid: &Vec<Vec<char>>) {
    let mut antenna_pos = HashMap::<char, Vec<(i32, i32)>>::new();
    grid.iter().enumerate().for_each(|(y, row)| {
       row.iter().enumerate().for_each(|(x, val)| {
           antenna_pos.entry(*val).or_insert_with(Vec::new).push((x as i32, y as i32));
       })
    });

    let w = grid[0].len();
    let h = grid.len();
    let mut antinodes = vec![vec![false; w]; h];
    let w = w as i32;
    let h = h as i32;

    for (c, same_freq) in antenna_pos {
        if c == '.' { continue; }
        for (i, (x0, y0)) in same_freq.iter().enumerate() {
            for (x1, y1) in same_freq.iter().skip(i+1) {
                let dx = x0 - x1;
                let dy = y0 - y1;

                let (x2, y2) = (x0 + dx, y0 + dy);
                let (x3, y3) = (x1 - dx, y1 - dy);

                if x2 >= 0 && x2 < w && y2 >= 0 && y2 < h {
                    antinodes[y2 as usize][x2 as usize] = true;
                }
                if x3 >= 0 && x3 < w && y3 >= 0 && y3 < h {
                    antinodes[y3 as usize][x3 as usize] = true;
                }
            }
        }
    }

    println!("Antinodes");
    antinodes.iter().for_each(|row| {
        println!("{:?}", row);
    });

    let count = antinodes.iter()
        .fold(0, |acc, vec| {
            vec.iter()
                .fold(acc,
                      |a, x| if *x { a + 1 } else { a }
                )
        });

    println!("Number of antinodes {}", count);
}

fn gcd(a: i32, b: i32) -> i32 {
    println!("GCD {} {}", a, b);
    if a == b || b == 0 {
        return a;
    }

    if a < 0 {
        return gcd(b, -a);
    }
    if a < b {
        return gcd(b, a);
    }


    let diff = a % b;
    return gcd(b, diff);
}

fn part2(grid: &Vec<Vec<char>>) {
    let mut antenna_pos = HashMap::<char, Vec<(i32, i32)>>::new();
    grid.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, val)| {
            antenna_pos.entry(*val).or_insert_with(Vec::new).push((x as i32, y as i32));
        })
    });

    let w = grid[0].len();
    let h = grid.len();
    let mut antinodes = vec![vec![false; w]; h];
    let w = w as i32;
    let h = h as i32;

    for (c, same_freq) in antenna_pos {
        if c == '.' { continue; }
        for (i, (x0, y0)) in same_freq.iter().enumerate() {
            for (x1, y1) in same_freq.iter().skip(i+1) {
                let dx = x0 - x1;
                let dy = y0 - y1;

                let stride = gcd(dx, dy);
                let (sx, sy) = (dx / stride, dy / stride);

                antinodes[*y0 as usize][*x0 as usize] = true;

                let (mut x2, mut y2) = (x0 + sx, y0 + sy);
                while x2 >= 0 && x2 < w && y2 >= 0 && y2 < h {
                    if y2 == 1 && x2 == 2 {
                        println!("Set incorrect node at {} ({} {}) ({} {}) ({} {} {})", c, x0, y0, x1, y1, sx, sy, stride);
                    }
                    antinodes[y2 as usize][x2 as usize] = true;
                    (x2, y2) = (x2 + sx, y2 + sy);
                }

                (x2, y2) = (x0 - sx, y0 - sy);
                while x2 >= 0 && x2 < w && y2 >= 0 && y2 < h {
                    if y2 == 1 && x2 == 2 {
                        println!("Set incorrect node at {} ({} {}) ({} {}) ({} {})", c, x0, y0, x1, y1, sx, sy);
                    }
                    antinodes[y2 as usize][x2 as usize] = true;
                    (x2, y2) = (x2 - sx, y2 - sy);
                }
            }
        }
    }

    println!("Antinodes");
    antinodes.iter().for_each(|row| {
        println!("{:?}", row);
    });

    let count = antinodes.iter()
        .fold(0, |acc, vec| {
            vec.iter()
                .fold(acc,
                      |a, x| if *x { a + 1 } else { a }
                )
        });

    println!("Number of antinodes {}", count);
}

fn main() {
    let lines: Vec<Vec<char>> = io::stdin()
        .lines()
        .enumerate()
        .map(|(y, line) | line.expect("Could not read line").chars().collect())
        .collect();
    lines.iter().for_each(|row| {
        println!("{:?}", row);
    });


    part1(&lines);
    part2(&lines);
}
