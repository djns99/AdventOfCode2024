use std::io;

fn part1(lines: &Vec<Vec<char>>) {

}


fn part2(lines: &Vec<Vec<char>>) {

}

fn main() {
    let lines: Vec<Vec<char>> = io::stdin()
        .lines()
        .map(|line| line.expect("Could not read line").chars().collect())
        .collect();
    lines.iter().for_each(|row| {
        println!("{:?}", row);
    });

    part1(&lines);
    part2(&lines);
}
