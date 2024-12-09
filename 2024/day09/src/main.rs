use std::{collections::BinaryHeap, fs};

const SPACE: usize = usize::MAX;

#[derive(Debug, Copy, Clone)]
struct Item {
    id: usize,
    pos: usize,
    len: usize,
}

fn checksum(disk: &[Item]) -> usize {
    let mut result = 0;
    let mut pos = 0;
    for d in disk {
        if d.id == SPACE {
            pos += d.len;
        } else {
            for _ in 0..d.len {
                result += pos * d.id;
                pos += 1;
            }
        }
    }
    result
}

fn part1(mut disk: Vec<Item>) -> usize {
    let mut result = Vec::new();
    let mut i = disk.len() - 1;
    let mut j = 0;

    while i >= j {
        if disk[i].id != SPACE {
            // just copy files
            result.push(disk[i]);
        } else {
            // we found space that needs to be filled: copy blocks from
            // files from the back
            let mut space_len = disk[i].len;
            while space_len > 0 {
                // first file from back
                if j >= i {
                    break;
                }
                let f = disk[j];

                if f.len <= space_len {
                    // file fits completely into space
                    space_len -= f.len;
                    result.push(f);
                    j += 2; // next file - skip space
                } else {
                    // file is larger than space: copy as many blocks as possible
                    result.push(Item {
                        id: f.id,
                        pos: result.len(),
                        len: space_len,
                    });
                    disk[j].len -= space_len;
                    space_len = 0;
                }
            }
        }

        i -= 1;
    }

    checksum(&result)
}

fn part2(mut disk: Vec<Item>, mut space_index: [BinaryHeap<usize>; 10]) -> usize {
    let mut j = 0;
    while j < disk.len() {
        // get next file from back
        let f = disk[j];

        // find space where the file would fit completely
        let mut sii = usize::MAX;
        let mut i = 0;
        for (k, si) in space_index.iter_mut().enumerate().skip(f.len - 1) {
            if let Some(np) = si.peek() {
                // find left-most space
                if *np > i {
                    i = *np;
                    sii = k;
                }
            }
        }

        if sii != usize::MAX && i > j {
            // move file
            disk[j].pos = disk[i].pos;

            // make space before the moved file larger
            disk[j + 1].len += disk[j].len;

            // make new space smaller
            disk[i].len -= disk[j].len;

            // update index
            space_index[sii].pop();
            if disk[i].len > 0 {
                space_index[(disk[i].len - 1).min(9)].push(i);
            }
        }

        j += 2; // next file - skip space
    }

    disk.sort_by_key(|i| i.pos); // stable sort!

    checksum(&disk)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let mut disk = Vec::new();
    let mut space_index = [const { BinaryHeap::new() }; 10];
    let bytes = input.trim().as_bytes();
    let mut i = bytes.len() - 1;
    loop {
        let id = if i % 2 == 0 { i / 2 } else { SPACE };
        let len = (bytes[i] - b'0') as usize;
        disk.push(Item { id, pos: i, len });
        if id == SPACE && len > 0 {
            space_index[len - 1].push(bytes.len() - i - 1);
        }
        if i == 0 {
            break;
        }
        i -= 1;
    }

    println!("{}", part1(disk.clone()));
    println!("{}", part2(disk, space_index));
}
