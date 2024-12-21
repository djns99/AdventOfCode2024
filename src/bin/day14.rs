use std::io;

#[derive(Debug)]
struct Robot {
    pos: (i64, i64),
    dir: (i64, i64),
}

fn part1(robots: &Vec<Robot>) {
    let (w, h) = (101, 103);
    let steps = 100;
    let mut quad_count = [0, 0, 0, 0, 0];
    for r in robots {
        let (pos, dir) = (r.pos, r.dir);
        let (x, y) = ((pos.0 + dir.0 * steps).rem_euclid(w), (pos.1 + dir.1 * steps).rem_euclid(h));

        let x_left = x < w/2;
        let x_right = x > w/2;
        let y_left = y < h/2;
        let y_right = y > h/2;
        let quad = match (x_left, x_right, y_left, y_right) {
            (true, false, true, false) => 0,
            (false, true, true, false) => 1,
            (true, false, false, true) => 2,
            (false, true, false, true) => 3,
            _ => 4
        };
        quad_count[quad] += 1;
        println!("Robot {:?} ends {} {}", r, x, y);
        println!("{:?} -> {}", (x_left, x_right, y_left, y_right), quad);
    }

    println!("Result {:?}", quad_count);
    println!("Result {}", quad_count[0] * quad_count[1] * quad_count[2] * quad_count[3])
}


fn part2(robots: &Vec<Robot>) {
    let (w, h) = (101, 103);
    let mut grid = vec![vec![0; w as usize]; h as usize];
    for steps in 0..10000 {
        grid.iter_mut().for_each(|x| x.fill(0));
        for r in robots {
            let (pos, dir) = (r.pos, r.dir);
            let (x, y) = ((pos.0 + dir.0 * steps).rem_euclid(w), (pos.1 + dir.1 * steps).rem_euclid(h));
            grid[y as usize][x as usize] += 1;
        }



        let mut flag = false;
        let lines = grid.iter().map(|row| {
            let line = row.iter().map(|&x| if x > 0 { "#" } else {" "}).collect::<String>();
            if line.contains("###############################") {
                flag = true;
            }
            line
        }).collect::<Vec<String>>();

        if flag {
            println!("====================={}=====================", steps);
            println!("{}", lines.join("\n"));
            println!("====================={}=====================\n\n\n\n", steps);
        }
    }
}

fn main() {
    let lines: Vec<Robot> = io::stdin()
        .lines()
        .map(|line| line.expect("Could not read line"))
        .map(|robot| {
            let split_to_tuple = |s: &str, split| { let v: Vec<i64> = s.split(split).map(|x| x.parse::<i64>().unwrap()).collect(); (v[0], v[1]) };
            let mut robot_iter = robot.split(&['=', ' ']);
            robot_iter.next();
            let pos = split_to_tuple(robot_iter.next().unwrap(), ',');
            robot_iter.next();
            let dir = split_to_tuple(robot_iter.next().unwrap(), ',');
            Robot {pos, dir}
        })
        .collect();
    lines.iter().for_each(|row| {
        println!("{:?}", row);
    });

    part1(&lines);
    part2(&lines);
}
