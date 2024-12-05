use std::io;

fn look_up_valid(chr_idx: usize, chr_prev: char) -> i32 {
    let nextmap = "MAS\0";
    let prevmap = "\0XMA";
    if chr_prev == nextmap.chars().nth(chr_idx).unwrap() {
        1
    } else if chr_prev == prevmap.chars().nth(chr_idx).unwrap() {
        -1
    } else {
        0
    }
}

fn check_prev_chr(lines: &Vec<Vec<char>>, map: &Vec<Vec<[i32; 4]>>, curr: usize, x: usize, y: usize, x_off: usize, y_off: usize) -> (i32, bool) {
    let w = lines[0].len();
    let valid_x = if x_off < 2 { x >= x_off } else { x + 1 < w };
    if valid_x && y >= y_off {
        let x = if x_off == 2 { x + 1 } else { x - x_off };
        let y = y - y_off;
        let dir = look_up_valid(curr, lines[y][x]);
        let prev_dir = map[y][x][y_off + (x_off * y_off)];
        let is_same_dir_as_prev = dir != 0 && dir == prev_dir;
        let is_last_in_dir = (dir == 1 && curr == 0) || (dir == -1 && curr == 3);
        let is_full_xmas = is_last_in_dir && is_same_dir_as_prev;
        let output_dir = if curr == 0 { -1 } else if curr == 3 { 1 } else if is_same_dir_as_prev { dir } else { 0 };
//         println!("x: {}, y: {} off x {}, off y {}", if x_off == 2 { x - 1 } else { x + x_off }, y + y_off, x_off, y_off);
//         println!("curr: {}", curr);
//         println!("dir: {}, prev_dir {:?}", dir, map[y][x]);
//         println!("is_same_dir_as_prev: {}", is_same_dir_as_prev);
//         println!("is_last_in_dir: {}", is_last_in_dir);
//         println!("is_last_in_dir: {}", is_last_in_dir);
//         println!("is_full_xmas: {}", is_full_xmas);
//         println!("output_dir: {}", output_dir);
        (output_dir, is_full_xmas)
    }
    else if curr == 0 {
        (-1, false)
    } else if curr == 3 {
        (1, false)
    } else {
        (0, false)
    }
}

fn check_valid(lines: &Vec<Vec<char>>, map: &Vec<Vec<[i32; 4]>>, x: usize, y: usize) -> [(i32, bool); 4] {
    let currmap = "XMAS";

    let curr = currmap.find(lines[y][x]);
    if curr.is_none() {
        return [(0, false); 4];
    }
    let curr = curr.unwrap();

    let res_x = check_prev_chr(lines, map, curr, x, y, 1, 0);
    let res_y = check_prev_chr(lines, map, curr, x, y, 0, 1);
    let res_z = check_prev_chr(lines, map, curr, x, y, 1, 1);
    let res_w = check_prev_chr(lines, map, curr, x, y, 2, 1);
    [res_x, res_y, res_z, res_w]
}

fn part1(lines: &Vec<Vec<char>>) {
    let width = lines[0].len();
    let height = lines.len();

    let mut ends = vec![vec![0; width]; height];
    let mut map = vec![vec![[0; 4]; width]; height];
    let mut count = 0;
    for y in 0..height {
        for x in 0..width {
            let [(res_x, fin_x), (res_y, fin_y), (res_z, fin_z), (res_w, fin_w)] = check_valid(&lines, &map, x, y);

            map[y][x] = [res_x, res_y, res_z, res_w];
            if fin_x {
                println!("x: {}, {}", x, y);
                ends[y][x] += 1;
            }
            if fin_y {
                println!("y: {}, {}", x, y);
                ends[y][x] += 1; }
            if fin_z { println!("z: {}, {}", x, y); ends[y][x] += 1; }
            if fin_w { println!("w: {}, {}", x, y); ends[y][x] += 1; }
            count += ends[y][x];
        }
    }
    ends.iter().for_each(|row| {
        println!("{:?}", row);
    });
    println!("{}", count);
}

fn diagonal_x(lines: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let [n,e,s,w] = [lines[y-1][x-1], lines[y-1][x+1], lines[y+1][x+1], lines[y+1][x-1]];
    let x1 = (n == 'M' && s == 'S') || (s == 'M' && n == 'S');
    let x2 = (e == 'M' && w == 'S') || (w == 'M' && e == 'S');
    if x1 && x2 {
                println!("{}.{}\n.{}.\n{}.{}\n", n, e, lines[y][x], w, s);
        }
    x1 && x2
}

fn orthogonal_x(lines: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
        let [n,e,s,w] = [lines[y][x-1], lines[y-1][x], lines[y][x+1], lines[y+1][x]];
        let x1 = (n == 'M' && s == 'S') || (s == 'M' && n == 'S');
        let x2 = (e == 'M' && w == 'S') || (w == 'M' && e == 'S');
        if x1 && x2 {
            println!(".{}.\n{}{}{}\n.{}.\n", n, e, lines[y][x], w, s);
        }
        x1 && x2
}

fn part2(lines: &Vec<Vec<char>>) {
    let width = lines[0].len();
    let height = lines.len();
    let mut count = 0;

    for y in 1..height-1 {
        for x in 1..width-1 {
            if lines[y][x] == 'A' {
                if diagonal_x(&lines, x, y) { count += 1; }
//                 if orthogonal_x(&lines, x, y) { count += 1; }
            }
        }
    }
    println!("{}", count);
}

fn main() {
    let lines: Vec<Vec<char>> = io::stdin()
        .lines()
        .map(|line| line.expect("Could not read line").chars().collect())
        .collect();
    lines.iter().for_each(|row| {
        println!("{:?}", row);
    });

//     part1(&lines);
    part2(&lines);
}
