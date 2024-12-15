use std::cmp::min;
use std::collections::HashMap;
use std::io;

fn sum_a_to_b(a: usize, b: usize) -> usize {
    (a + b) * (b - a + 1) / 2
}

fn part1(mut diskmap: Vec<usize>) {
    let num_files = (diskmap.len() + 1) / 2;
    let mut last_file = num_files - 1;
    let mut check_sum = 0;
    let mut disk_offset = 0;
    for i in 0..(num_files-1) {
        let segment_count = diskmap[i * 2];
        if segment_count == 0 {
            break; // Reached some moved files
        }
        // println!("Processing {} with {} segments", i, segment_count);
        // println!("Checksum\n{} += {} * {} == {}", check_sum , i , sum_a_to_b(disk_offset, disk_offset + segment_count - 1), check_sum + i * sum_a_to_b(disk_offset, disk_offset + segment_count - 1));
        check_sum += i * sum_a_to_b(disk_offset, disk_offset + segment_count - 1);
        disk_offset += segment_count;

        let mut free_count = diskmap[i * 2 + 1];
        while free_count > 0 && last_file > i {
            let blocks_to_move = min(free_count, diskmap[last_file * 2]);
            if blocks_to_move == 0 {
                last_file -= 1;
                continue;
            }

            // println!("Checksum Move\n{} += {} * {} == {}", check_sum , last_file , sum_a_to_b(disk_offset, disk_offset + blocks_to_move - 1), check_sum + last_file * sum_a_to_b(disk_offset, disk_offset + blocks_to_move - 1));
            check_sum += last_file * sum_a_to_b(disk_offset, disk_offset + blocks_to_move - 1);
            disk_offset += blocks_to_move;
            free_count -= blocks_to_move;
            diskmap[last_file * 2] -= blocks_to_move;
        }
    }

    println!("Checksum {}", check_sum);
}


fn part2(mut diskmap: Vec<usize>) {
    let mut diskoff: Vec<usize> = diskmap.iter().scan(0 as usize, |state, &x| {*state += x; Some(*state - x)}).collect();
    println!("Scan {:?}", diskoff);
    let num_files = (diskmap.len() + 1) / 2;
    let mut check_sum = 0;
    'next_file: for fileid in (0..num_files).rev() {
        let segment_size = diskmap[fileid*2];
        for mv in 0..fileid {
            let space_offset = mv*2+1;
            if segment_size <= diskmap[space_offset] {
                println!("Moving segment {} of size {} to gap {} offset {}", fileid, segment_size, mv, diskoff[space_offset]);
                check_sum += fileid * sum_a_to_b(diskoff[space_offset], diskoff[space_offset] + segment_size - 1);
                diskmap[space_offset] -= segment_size;
                diskoff[space_offset] += segment_size;
                continue 'next_file;
            }
        }
        check_sum += fileid * sum_a_to_b(diskoff[fileid*2], diskoff[fileid*2] + segment_size - 1);
    }
    println!("Checksum {}", check_sum);
}

fn main() {
    let diskmap: Vec<usize> = io::stdin()
        .lines()
        .map(|line| line.expect("Could not read line").chars().map(|x: char| x.to_string().parse::<usize>().unwrap()).collect())
        .next().unwrap();
    println!("Input {:?}", diskmap);


    part1(diskmap.clone());
    part2(diskmap);
}
