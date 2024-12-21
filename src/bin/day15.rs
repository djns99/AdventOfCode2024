use std::io;
use std::mem::swap;

fn try_step(grid: &mut Vec<Vec<char>>, pos: (usize, usize), dir: (isize, isize)) -> (usize, usize) {
    let newpos = (pos.0 as isize + dir.0, pos.1 as isize + dir.1);
    let newpos = (newpos.0 as usize, newpos.1 as usize);
    let (x, y) = newpos;
    // Try clear the way if we can
    if grid[y][x] == 'O' {
        try_step(grid, newpos, dir);
    }
    match grid[y][x] {
        '.' => {
            grid[y][x] = grid[pos.1][pos.0];
            grid[pos.1][pos.0] = '.';
            newpos // success
        }
        _ => {
            pos // No change
        }
    }
}

fn part1(mut grid: Vec<Vec<char>>, instructions: &Vec<char>) {
    let mut robot_pos = grid.iter()
        .enumerate()
        .filter_map(|(y, row)| Some((y, row.iter().position(|&x| x == '@')?)))
        .next().unwrap();
    println!("Robot starting at {:?}", robot_pos);

    for &ins in instructions {
        let dir = match ins {
            '<' => (-1, 0),
            '>' => (1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => (0, 0)
        };

        robot_pos = try_step(&mut grid, robot_pos, dir);
    }
    println!("Final position");
    grid.iter().for_each(|row| {
        println!("{:?}", row.iter().collect::<String>());
    });

    let count = grid.iter().enumerate()
        .fold(0, |acc, (y, vec)| {
            vec.iter().enumerate()
                .fold(acc,
                      |a, (x, &c) | if c == 'O' { a + x + y * 100 } else { a }
                )
        });

    println!("GPS {}", count);
}


fn part2(grid: &Vec<Vec<char>>, instructions: &Vec<char>) {}

fn main() {
    let lines: Vec<Vec<char>> = io::stdin()
        .lines()
        .map(|line| line.expect("Could not read line").chars().collect())
        .collect();

    let iter = lines.into_iter();
    let grid: Vec<Vec<char>> = iter.clone().take_while(|x| !x.is_empty()).collect();
    let instructions: Vec<char> = iter.skip_while(|x| !x.is_empty()).flatten().collect();

    grid.iter().for_each(|row| {
        println!("{:?}", row);
    });


    part1(grid.clone(), &instructions);
    part2(&grid, &instructions);
}
