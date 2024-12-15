use std::collections::HashSet;
use std::io;

fn go_to_obstacle(grid: &Vec<Vec<bool>>, visited: &mut Vec<Vec<bool>>, pos: [i32; 4]) -> [i32; 4] {
    let w = grid[0].len() as i32;
    let h = grid.len() as i32;
    let [mut x, mut y, xdir, ydir] = pos;
    while grid[y as usize][x as usize] {
        visited[y as usize][x as usize] = true;

        y += ydir;
        x += xdir;

        if y < 0 || y >= h || x < 0 || x >= w {
            return [-1, -1, 0, 0];
        }
    }
    [x - xdir, y - ydir, -ydir, xdir]
}

fn run_path(grid: &Vec<Vec<bool>>, visited: &mut Vec<Vec<bool>>, start: (i32, i32)) -> bool {
    let mut seen = HashSet::<[i32; 4]>::new();

    let mut pos = [start.0, start.1, 0, -1];
    while pos[0] >= 0 {
        pos = go_to_obstacle(&grid, visited, pos);
        if !seen.insert(pos) {
            return true;
        }
    }

    return false;
}

fn part1(grid: &Vec<Vec<bool>>, start: (i32, i32)) {
    let w = grid[0].len();
    let h = grid.len();
    let mut visited = vec![vec![false; w]; h];

    run_path(&grid, &mut visited, start);

    println!("Visited map");
    visited.iter().for_each(|row| {
        println!("{:?}", row);
    });
    let count = visited.iter()
        .fold(0, |acc, vec| {
            vec.iter()
                .fold(acc,
                     |a, x| if *x { a + 1 } else { a }
                )
        });
    println!("Total squared {}", count);
}

fn part2(grid: &Vec<Vec<bool>>, start: (i32, i32)) {
    let mut grid = grid.clone();
    let w = grid[0].len();
    let h = grid.len();
    let mut visited = vec![vec![false; w]; h];

    run_path(&grid, &mut visited, start);

    let mut obstacle_visited = vec![vec![false; w]; h];
    let mut count = 0;
    visited.iter().enumerate()
        .for_each(|(y, row)| {
            row.iter().enumerate()
                .for_each(|(x, val)| {
                    if *val {
                        grid[y][x] = false;
                        if run_path(&grid, &mut obstacle_visited, start) {
                            count += 1;
                        }
                        grid[y][x] = true;
                        obstacle_visited.iter_mut().for_each(|x| x.fill(false));
                    }
                })
        });

    println!("Total loops {}", count);
}

fn main() {
    let mut start = (0,0);
    let lines: Vec<Vec<bool>> = io::stdin()
        .lines()
        .enumerate()
        .map(|(y, line) | line.expect("Could not read line").chars().enumerate().map(|(x, c)| {if(c == '^') {start = (x as i32, y as i32);} c != '#'}).collect())
        .collect();
    lines.iter().for_each(|row| {
        println!("{:?}", row);
    });


    part1(&lines, start);
    part2(&lines, start);
}
