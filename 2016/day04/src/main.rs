use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let rooms = input
        .lines()
        .map(|l| {
            let (s, hash) = l.split_once('[').unwrap();
            let hash = &hash[0..hash.len() - 1];
            let (room, id) = s.split_at(s.rfind('-').unwrap() + 1);
            (
                room.strip_suffix('-').unwrap(),
                id.parse::<u64>().unwrap(),
                hash,
            )
        })
        .collect::<Vec<_>>();

    let mut room_part2 = None;
    let mut sum = 0;
    for r in rooms {
        let mut chars: HashMap<char, usize> = HashMap::new();
        for c in r.0.chars() {
            if c.is_ascii_alphabetic() {
                *chars.entry(c).or_default() += 1;
            }
        }
        let mut chars = chars.into_iter().collect::<Vec<_>>();
        chars.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        let hash = String::from_iter(chars.into_iter().take(5).map(|c| c.0));
        if hash == r.2 {
            sum += r.1;
        }

        let mut decoded = String::new();
        for c in r.0.chars() {
            if c == '-' {
                decoded.push(' ');
            } else {
                let d = ((((c as u64 - b'a' as u64) + r.1) % 26) as u8 + b'a') as char;
                decoded.push(d);
            }
        }
        if decoded == "northpole object storage" {
            room_part2 = Some(r.1);
        }
    }

    println!("{}", sum);
    println!("{}", room_part2.unwrap());
}
