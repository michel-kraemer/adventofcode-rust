use std::fs;

const NORTHPOLE_OBJECT_STORAGE: &[u8; 24] = b"northpole object storage";

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let mut counts = [0; 26];
    let mut sum = 0;
    let mut object_storage_id = 0;
    for l in input.lines() {
        let (s, hash) = l.split_once('[').unwrap();
        let hash = &hash[0..hash.len() - 1];
        let (room, id) = s.split_at(s.rfind('-').unwrap());
        let id = id[1..].parse::<u64>().unwrap();

        // count character frequencies
        let mut max = 0;
        counts.fill(0);
        for c in room.bytes() {
            if c.is_ascii_alphabetic() {
                let e = &mut counts[(c - b'a') as usize];
                *e += 1;
                max = max.max(*e);
            }
        }

        // part 1: validate hash
        let mut hash_bytes = hash.bytes();
        let mut prev_h = hash_bytes.next().unwrap();
        let mut good = counts[(prev_h - b'a') as usize] == max; // check first char
        if good {
            for h in hash_bytes {
                let c = counts[(h - b'a') as usize];
                // there must be a count, it must be less than or equal to the
                // previous count, and if it's equal, the character must be less
                // than the previous one
                if c == 0 || c > max || (c == max && h <= prev_h) {
                    good = false;
                    break;
                }
                max = c;
                prev_h = h;
            }
            if good {
                sum += id;
            }
        }

        // part 2: decrypt room name
        if object_storage_id == 0
            && room.bytes().zip(NORTHPOLE_OBJECT_STORAGE).all(|(r, e)| {
                let re = if r == b'-' {
                    b' '
                } else {
                    (((r - b'a') as u64 + id) % 26) as u8 + b'a'
                };
                *e == re
            })
        {
            object_storage_id = id;
        }
    }

    println!("{sum}");
    println!("{object_storage_id}");
}
