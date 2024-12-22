use std::io;
use std::mem::swap;

fn advance(pos: (usize, usize), dir: (isize, isize)) -> (usize, usize) {
    let newpos = (pos.0 as isize + dir.0, pos.1 as isize + dir.1);
    (newpos.0 as usize, newpos.1 as usize)
}

fn scan_dir(grid: &Vec<Vec<char>>, pos: (usize, usize), dir: (isize, isize)) -> bool {
    let (mut x, mut y) = pos;
    match grid[y][x] {
        '#' => false,
        '.' => true,
        v => {
            let is_left = v == '[';
            if is_left {
                x += 1;
            }
            scan_dir(grid, advance((x-1, y), dir), dir) && scan_dir(grid, advance((x, y), dir), dir)
        }
        _ => panic!(),
    }
}

fn try_step_block(grid: &mut Vec<Vec<char>>, pos: (usize, usize), dir: (isize, isize)) -> (usize, usize) {
    let (mut x, y) = pos;
    if !scan_dir(grid, pos, dir) {
        return pos
    }

    let is_left = grid[y][x] == '[';
    if is_left {
        x += 1;
    }
    let left_pos = (x - 1, y);
    let right_pos = (x, y);
    let left_step = try_step(grid, left_pos, dir);
    let right_step = try_step(grid, right_pos, dir);
    if is_left {
        left_step
    } else {
        right_step
    }
}

fn try_step(grid: &mut Vec<Vec<char>>, pos: (usize, usize), dir: (isize, isize)) -> (usize, usize) {
    let newpos = advance(pos, dir);
    let (x, y) = newpos;
    // Try clear the way if we can
    match grid[y][x] {
        'O' => {
            try_step(grid, newpos, dir);
        }
        '[' | ']' => {
            if dir.1 == 0 {
                try_step(grid, newpos, dir); // Moving sideways, just step normally
            } else {
                try_step_block(grid, newpos, dir); // Need to do a coupled step
            }
        }
        _ => {}
    };
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
        .filter_map(|(y, row)| Some((row.iter().position(|&x| x == '@')?, y)))
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

        // grid.iter().for_each(|row| {
        //     println!("{:?}", row.iter().collect::<String>());
        // });
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
                      |a, (x, &c)| if c == 'O' || c == '[' { a + x + y * 100 } else { a },
                )
        });

    println!("GPS {}", count);
}


fn part2(grid: &Vec<Vec<char>>, instructions: &Vec<char>) {
    let grid: Vec<Vec<char>> = grid.into_iter().map(|x| x.into_iter().map(|&c| match c {
        'O' => ['[', ']'],
        '@' => ['@', '.'],
        v => [v, v],
    }).flatten().collect()).collect();
    grid.iter().for_each(|row| {
        println!("{:?}", row);
    });

    part1(grid, instructions);
}

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
