use std::fs;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Instruction {
    SwapPosition(usize, usize),
    SwapLetter(u8, u8),
    RotateLeft(usize),
    RotateRight(usize),
    RotateBasedOnLetter(u8),
    ReversePositions(usize, usize),
    MovePosition(usize, usize),
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let instructions = input
        .lines()
        .map(|l| {
            if l.starts_with("swap position") {
                let mut params = l.split_ascii_whitespace();
                let x = params.nth(2).unwrap().parse::<usize>().unwrap();
                let y = params.next_back().unwrap().parse::<usize>().unwrap();
                Instruction::SwapPosition(x, y)
            } else if l.starts_with("swap letter") {
                let mut params = l.split_ascii_whitespace();
                let x = params.nth(2).unwrap().bytes().next().unwrap();
                let y = params.next_back().unwrap().bytes().next().unwrap();
                Instruction::SwapLetter(x, y)
            } else if l.starts_with("rotate left") {
                let mut params = l.split_ascii_whitespace();
                let x = params.nth(2).unwrap().parse::<usize>().unwrap();
                Instruction::RotateLeft(x)
            } else if l.starts_with("rotate right") {
                let mut params = l.split_ascii_whitespace();
                let x = params.nth(2).unwrap().parse::<usize>().unwrap();
                Instruction::RotateRight(x)
            } else if l.starts_with("rotate based on position of letter") {
                let mut params = l.split_ascii_whitespace();
                let x = params.next_back().unwrap().bytes().next().unwrap();
                Instruction::RotateBasedOnLetter(x)
            } else if l.starts_with("reverse positions") {
                let mut params = l.split_ascii_whitespace();
                let x = params.nth(2).unwrap().parse::<usize>().unwrap();
                let y = params.next_back().unwrap().parse::<usize>().unwrap();
                Instruction::ReversePositions(x, y)
            } else if l.starts_with("move position") {
                let mut params = l.split_ascii_whitespace();
                let x = params.nth(2).unwrap().parse::<usize>().unwrap();
                let y = params.next_back().unwrap().parse::<usize>().unwrap();
                Instruction::MovePosition(x, y)
            } else {
                panic!("Unknown instruction: {l}");
            }
        })
        .collect::<Vec<_>>();

    for part1 in [true, false] {
        let mut instructions = instructions.clone();
        let mut s = if part1 {
            b"abcdefgh".to_vec()
        } else {
            b"fbgdceah".to_vec()
        };

        if !part1 {
            instructions.reverse();
        }

        for i in instructions {
            match i {
                Instruction::SwapPosition(x, y) => {
                    s.swap(x, y);
                }
                Instruction::SwapLetter(x, y) => {
                    s = s
                        .into_iter()
                        .map(|c| {
                            if c == x {
                                b'#'
                            } else if c == y {
                                x
                            } else {
                                c
                            }
                        })
                        .map(|c| if c == b'#' { y } else { c })
                        .collect();
                }
                Instruction::RotateLeft(x) => {
                    if part1 {
                        s.rotate_left(x);
                    } else {
                        s.rotate_right(x);
                    }
                }
                Instruction::RotateRight(x) => {
                    if part1 {
                        s.rotate_right(x);
                    } else {
                        s.rotate_left(x);
                    }
                }
                Instruction::RotateBasedOnLetter(x) => {
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
                }
                Instruction::ReversePositions(mut x, mut y) => {
                    while x < y {
                        s.swap(x, y);
                        x += 1;
                        y -= 1;
                    }
                }
                Instruction::MovePosition(x, y) => {
                    if part1 {
                        let c = s.remove(x);
                        s.insert(y, c);
                    } else {
                        let c = s.remove(y);
                        s.insert(x, c);
                    }
                }
            }
        }

        println!("{}", s.into_iter().map(|b| b as char).collect::<String>());
    }
}
