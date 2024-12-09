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

fn part1(grid: &Vec<Vec<bool>>, start: (i32, i32)) {
    let w = grid[0].len();
    let h = grid.len();
    let mut visited = vec![vec![false; w]; h];

    let mut seen = HashSet::<[i32; 4]>::new();

    let mut pos = [start.0, start.1, 0, -1];
    while pos[0] >= 0 {
        pos = go_to_obstacle(&grid, &mut visited, pos);
        if !seen.insert(pos) {
            break;
        }
    }

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
