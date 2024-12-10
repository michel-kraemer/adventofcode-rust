use std::{cmp::Reverse, collections::BinaryHeap, fs};

#[derive(Debug, Copy, Clone)]
struct Item {
    pos: usize,
    len: usize,
}

fn part1(mut disk: Vec<Item>) -> usize {
    let mut result = 0;
    let mut i = 0;
    let mut j = disk.len() - 1;

    while i <= j {
        if i % 2 == 0 {
            // file: add checksum
            for k in 0..disk[i].len {
                result += (disk[i].pos + k) * i / 2;
            }
        } else {
            // space: add checksum of blocks from the back
            for k in 0..disk[i].len {
                result += (disk[i].pos + k) * j / 2;
                disk[j].len -= 1;
                if disk[j].len == 0 {
                    j -= 2;
                }
            }
        }

        i += 1;
    }

    result
}

fn part2(disk: Vec<Item>, mut space_index: [BinaryHeap<Reverse<usize>>; 10]) -> usize {
    let mut result = 0;
    let mut j = disk.len() - 1;

    while j > 0 {
        // get next file from back
        let f = disk[j];

        // find space where the file would fit completely
        let mut sii = usize::MAX;
        let mut i = usize::MAX;
        for (k, si) in space_index.iter_mut().enumerate().skip(f.len) {
            if let Some(Reverse(np)) = si.peek() {
                // find left-most space
                if *np < i {
                    i = *np;
                    sii = k;
                }
            }
        }

        let pos = if sii != usize::MAX && i < f.pos {
            // move file
            space_index[sii].pop();
            let new_len = sii - f.len;
            if new_len > 0 {
                space_index[new_len].push(Reverse(i + f.len));
            }
            i
        } else {
            // file has to stay where it is
            f.pos
        };

        // add checksum
        for k in 0..f.len {
            result += (pos + k) * j / 2;
        }

        j -= 2; // next file - skip space
    }

    result
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let mut disk = Vec::new();
    let mut space_index = [const { BinaryHeap::new() }; 10];
    let bytes = input.trim().as_bytes();
    let mut pos = 0;
    for b in bytes {
        let len = (b - b'0') as usize;
        disk.push(Item { pos, len });
        if disk.len() % 2 == 0 && len > 0 {
            space_index[len].push(Reverse(pos));
        }
        pos += len;
    }

    println!("{}", part1(disk.clone()));
    println!("{}", part2(disk, space_index));
}
