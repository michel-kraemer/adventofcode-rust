use std::fs;

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let mut instructions = input.lines().collect::<Vec<_>>();

        let mut s = (if part1 { "abcdefgh" } else { "fbgdceah" })
            .chars()
            .collect::<Vec<_>>();

        if !part1 {
            instructions.reverse();
        }

        for i in instructions {
            if i.starts_with("swap position") {
                let mut params = i.split(' ');
                let x = params.nth(2).unwrap().parse::<usize>().unwrap();
                let y = params.next_back().unwrap().parse::<usize>().unwrap();
                s.swap(x, y);
            } else if i.starts_with("swap letter") {
                let mut params = i.split(' ');
                let x = params.nth(2).unwrap().chars().next().unwrap();
                let y = params.next_back().unwrap().chars().next().unwrap();
                s = s
                    .into_iter()
                    .map(|c| if c == x { '#' } else { c })
                    .map(|c| if c == y { x } else { c })
                    .map(|c| if c == '#' { y } else { c })
                    .collect();
            } else if i.starts_with("rotate left") {
                let mut params = i.split(' ');
                let x = params.nth(2).unwrap().parse::<usize>().unwrap();
                if part1 {
                    s.rotate_left(x);
                } else {
                    s.rotate_right(x);
                }
            } else if i.starts_with("rotate right") {
                let mut params = i.split(' ');
                let x = params.nth(2).unwrap().parse::<usize>().unwrap();
                if part1 {
                    s.rotate_right(x);
                } else {
                    s.rotate_left(x);
                }
            } else if i.starts_with("rotate based on position of letter") {
                let mut params = i.split(' ');
                let x = params.next_back().unwrap().chars().next().unwrap();

                if part1 {
                    let idx = s.iter().position(|c| *c == x).unwrap();
                    let len = s.len();
                    s.rotate_right((1 + idx + (if idx >= 4 { 1 } else { 0 })) % len);
                } else {
                    let mut cl = s.clone();
                    loop {
                        cl.rotate_left(1);
                        let mut cl2 = cl.clone();
                        let idx = cl2.iter().position(|c| *c == x).unwrap();
                        let len = cl2.len();
                        cl2.rotate_right((1 + idx + (if idx >= 4 { 1 } else { 0 })) % len);
                        if cl2 == s {
                            s = cl;
                            break;
                        }
                    }
                }
            } else if i.starts_with("reverse positions") {
                let mut params = i.split(' ');
                let mut x = params.nth(2).unwrap().parse::<usize>().unwrap();
                let mut y = params.next_back().unwrap().parse::<usize>().unwrap();
                while x < y {
                    s.swap(x, y);
                    x += 1;
                    y -= 1;
                }
            } else if i.starts_with("move position") {
                let mut params = i.split(' ');
                let x = params.nth(2).unwrap().parse::<usize>().unwrap();
                let y = params.next_back().unwrap().parse::<usize>().unwrap();
                if part1 {
                    let c = s.remove(x);
                    s.insert(y, c);
                } else {
                    let c = s.remove(y);
                    s.insert(x, c);
                }
            }
        }

        println!("{}", String::from_iter(s));
    }
}
