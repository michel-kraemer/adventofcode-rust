use std::fs;

const PAD1: [[char; 3]; 3] = [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']];

const PAD2: [[char; 5]; 5] = [
    ['.', '.', '1', '.', '.'],
    ['.', '2', '3', '4', '.'],
    ['5', '6', '7', '8', '9'],
    ['.', 'A', 'B', 'C', '.'],
    ['.', '.', 'D', '.', '.'],
];

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let mut x1 = 1;
    let mut y1 = 1;
    let mut x2 = 0;
    let mut y2 = 2;

    let mut code1 = String::new();
    let mut code2 = String::new();
    for i in input.lines().map(|l| l.as_bytes()) {
        for d in i {
            match d {
                b'L' => {
                    if x1 > 0 && PAD1[y1][x1 - 1] != '.' {
                        x1 -= 1
                    }
                    if x2 > 0 && PAD2[y2][x2 - 1] != '.' {
                        x2 -= 1
                    }
                }
                b'R' => {
                    if x1 < PAD1[0].len() - 1 && PAD1[y1][x1 + 1] != '.' {
                        x1 += 1
                    }
                    if x2 < PAD2[0].len() - 1 && PAD2[y2][x2 + 1] != '.' {
                        x2 += 1
                    }
                }
                b'U' => {
                    if y1 > 0 && PAD1[y1 - 1][x1] != '.' {
                        y1 -= 1
                    }
                    if y2 > 0 && PAD2[y2 - 1][x2] != '.' {
                        y2 -= 1
                    }
                }
                b'D' => {
                    if y1 < PAD1.len() - 1 && PAD1[y1 + 1][x1] != '.' {
                        y1 += 1
                    }
                    if y2 < PAD2.len() - 1 && PAD2[y2 + 1][x2] != '.' {
                        y2 += 1
                    }
                }
                _ => unreachable!(),
            }
        }
        code1.push(PAD1[y1][x1]);
        code2.push(PAD2[y2][x2]);
    }

    println!("{code1}");
    println!("{code2}");
}
