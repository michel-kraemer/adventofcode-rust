use std::fs;

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let instructions = input
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let pad = if part1 {
            vec![
                vec!['1', '2', '3'],
                vec!['4', '5', '6'],
                vec!['7', '8', '9'],
            ]
        } else {
            vec![
                vec!['.', '.', '1', '.', '.'],
                vec!['.', '2', '3', '4', '.'],
                vec!['5', '6', '7', '8', '9'],
                vec!['.', 'A', 'B', 'C', '.'],
                vec!['.', '.', 'D', '.', '.'],
            ]
        };

        let mut x = if part1 { 1 } else { 0 };
        let mut y = if part1 { 1 } else { 2 };

        let mut code = String::new();
        for i in instructions {
            for d in i {
                match d {
                    'L' => {
                        if x > 0 && pad[y][x - 1] != '.' {
                            x -= 1
                        }
                    }
                    'R' => {
                        if x < pad[0].len() - 1 && pad[y][x + 1] != '.' {
                            x += 1
                        }
                    }
                    'U' => {
                        if y > 0 && pad[y - 1][x] != '.' {
                            y -= 1
                        }
                    }
                    'D' => {
                        if y < pad.len() - 1 && pad[y + 1][x] != '.' {
                            y += 1
                        }
                    }
                    _ => unreachable!(),
                }
            }
            code.push(pad[y][x]);
        }

        println!("{}", code);
    }
}
