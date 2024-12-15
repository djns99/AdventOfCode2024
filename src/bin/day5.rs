use std::cmp::Ordering;
use std::io;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::mem::swap;
use std::thread::sleep;
use std::time::Duration;

#[derive(Clone, Debug)]
struct MapCache {
    successors: Vec<i32>,
    depth: i32,
    has_pred: bool,
}

fn part1(lines: &Vec<String>) {
    let mut map = HashMap::<i32, MapCache>::new();
    let mut l_iter = lines.iter();
    let parse_int = |x: &str| x.parse::<i32>().unwrap();
    for order in l_iter.by_ref().take_while(|x| !x.is_empty()) {
        let [left, right] = order.split('|').map(parse_int).take(2).collect::<Vec<i32>>().try_into().unwrap();
        map.entry(left).or_insert(MapCache{successors:vec![], depth:0, has_pred: false}).successors.push(right);
    }

    let mut seen = HashSet::<i32>::new();
    let mut count = 0;
    for order in l_iter {
        let mut is_valid = true;
        let mut len = 0;
        let split_iter = order.split(',').map(parse_int);
        'outer: for o in split_iter.clone() {
            let map_entry = map.get(&o);
            if map_entry.is_some() {
                for s in &map_entry.unwrap().successors {
                    if seen.contains(&s) {
                        is_valid = false;
                        break 'outer;
                    }
                }
            }

            seen.insert(o);
            len += 1;
        }

        if is_valid {
            let mid = split_iter.skip(len / 2).next().unwrap();
            count += mid;
        }

        seen.clear();
    }
    println!("Found {} valid", count);
}

// fn part2(lines: &Vec<String>) {
//     let mut map = HashMap::<i32, MapCache>::new();
//     let mut l_iter = lines.iter();
//     let parse_int = |x: &str| x.parse::<i32>().unwrap();
//     for order in l_iter.by_ref().take_while(|x| !x.is_empty()) {
//         let [left, right] = order.split('|').map(parse_int).take(2).collect::<Vec<i32>>().try_into().unwrap();
//         map.entry(left).or_insert(MapCache{successors:vec![], depth:0, has_pred: false}).successors.push(right);
//     }
//
//     let mut seen = HashMap::<i32, usize>::new();
//     let mut count = 0;
//     for order in l_iter {
//         let mut is_valid = true;
//         let mut len = 0;
//         let mut split = order.split(',').map(parse_int).collect::<Vec<i32>>();
//         for pos in 0..split.len() {
//             let o = split[pos];
//             let map_entry = map.get(&o);
//             if map_entry.is_some() {
//                 for s in &map_entry.unwrap().successors {
//                     match seen.get(&s) {
//                         Some(x) => { is_valid = false; split.swap(pos, *x);}
//                         None => {}
//                     }
//                 }
//             }
//
//             seen.insert(o, pos);
//         }
//
//         if !is_valid {
//             let mid = split[split.len() / 2];
//             count += mid;
//         }
//
//         seen.clear();
//     }
//     println!("Found {} valid", count);
// }

fn build_ordering(rules: &HashMap<i32, MapCache>, input: &Vec<i32>) -> HashMap<i32, MapCache> {
    let valid_keys =  HashSet::<i32>::from_iter(input.iter().map(|x|*x));
    let mut filtered_rules= HashMap::<i32, MapCache>::new();
    valid_keys.iter().for_each(
        |x| {
            let mut filter_cache = rules[x].clone();
            filter_cache.successors = filter_cache.successors.into_iter().filter(|x| valid_keys.contains(x)).collect();
            filtered_rules.insert(*x, filter_cache);
        }
    );
    input.iter().for_each(
        |x| {
            let numscs = filtered_rules[x].successors.len();
            for s in 0..numscs {
                let s = filtered_rules[x].successors[s];
                filtered_rules.get_mut(&s).unwrap().has_pred = true;
            }
        }
    );


    println!("Map is {:?}", filtered_rules);
    let mut queue: HashSet<i32> = valid_keys.iter().filter(|x| !filtered_rules[x].has_pred).map(|x|*x).collect();
    let mut nextqueue = HashSet::<i32>::new();
    println!("Root set is {:?}", queue);
    let mut depth = 0;
    while !queue.is_empty() {
        for next in &queue {
            filtered_rules.entry(*next).and_modify(|y| {
                println!("Value {} pushing {:?}", next, y.successors);
                y.depth = depth;
                y.successors.iter().for_each(|x| {nextqueue.insert(*x);});
            });
        }
        depth += 1;
        println!("Map is {:?}", filtered_rules);
        swap(&mut queue, &mut nextqueue);
        nextqueue.clear();
    }
    filtered_rules
}

fn part2(lines: &Vec<String>) {
    let mut map = HashMap::<i32, MapCache>::new();
    let mut l_iter = lines.iter();
    let parse_int = |x: &str| x.parse::<i32>().unwrap();
    let default_map = MapCache{successors:vec![], depth:0, has_pred: false};
    for order in l_iter.by_ref().take_while(|x| !x.is_empty()) {
        let [left, right] = order.split('|').map(parse_int).take(2).collect::<Vec<i32>>().try_into().unwrap();
        map.entry(left).or_insert(default_map.clone()).successors.push(right);
    }

    let mut seen = HashSet::<i32>::new();
    let mut count = 0;
    for order in l_iter {
        let mut is_valid = true;
        let split_iter = order.split(',').map(parse_int);
        'outer: for o in split_iter.clone() {
            let map_entry = map.get(&o);
            if map_entry.is_some() {
                for s in &map_entry.unwrap().successors {
                    if seen.contains(&s) {
                        is_valid = false;
                        break 'outer;
                    }
                }
            }

            seen.insert(o);
        }

        if !is_valid {
            let mut seq = split_iter.collect::<Vec<i32>>();
            let map = build_ordering(&map, &seq);
            seq.sort_by(|ogx, ogy|
                {
                    let x = map.get(ogx);
                    let y = map.get(ogy);
                    if x.is_none() && y.is_none() {
                        return Ordering::Equal;
                    }
                    if x.is_none() {
                        return Ordering::Less;
                    }
                    if y.is_none() {
                        return Ordering::Greater;
                    }

                    if x.unwrap().depth == y.unwrap().depth {
                        return ogx.cmp(ogy);
                    }

                    x.unwrap().depth.cmp(&y.unwrap().depth)
                }
            );
            // println!("Sorted seq {:?}", seq);
            count += seq[seq.len() / 2].clone();
        }

        seen.clear();
    }
    println!("Found {} valid", count);
}

fn main() {
    let lines: Vec<String> = io::stdin()
        .lines()
        .map(|line| line.expect("Could not read line"))
        .collect();
    lines.iter().for_each(|row| {
        println!("{:?}", row);
    });

    part1(&lines);
    part2(&lines);
}
