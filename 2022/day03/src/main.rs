use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();

    // part 1
    let mut total = 0u64;
    for l in &lines {
        let c1 = &l.as_bytes()[0..l.len() / 2];
        let c2 = &l.as_bytes()[l.len() / 2..];
        for b in c1 {
            if c2.contains(b) {
                if b.is_ascii_lowercase() {
                    total += (*b - b'a' + 1) as u64;
                } else {
                    total += (*b - b'A' + 27) as u64;
                }
            }
        }
    }
    println!("{total}");

    // part 2
    let mut total = 0u64;
    for l in lines.chunks(3) {
        let l1 = l[0].as_bytes();
        let l2 = l[1].as_bytes();
        let l3 = l[2].as_bytes();
        for b in l1 {
            if l2.contains(b) && l3.contains(b) {
                if b.is_ascii_lowercase() {
                    total += (*b - b'a' + 1) as u64;
                } else {
                    total += (*b - b'A' + 27) as u64;
                }
            }
        }
    }
    println!("{total}");
}
