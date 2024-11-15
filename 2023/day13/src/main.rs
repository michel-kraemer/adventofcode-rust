use std::fs;

fn transpose(pattern: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut new_pattern = vec![vec![' '; pattern.len()]; pattern[0].len()];
    for (y, row) in pattern.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            new_pattern[x][y] = *cell;
        }
    }
    new_pattern
}

fn find(p: &[Vec<char>], part1: bool) -> Option<usize> {
    for x in 1..p[0].len() {
        let mut dx = 1;
        let mut found_smudge = part1;
        'l: while dx <= x && x + dx <= p[0].len() {
            for row in p {
                if row[x - dx] != row[x + dx - 1] {
                    if found_smudge {
                        break 'l;
                    } else {
                        found_smudge = true;
                    }
                }
            }
            if found_smudge && (x - dx == 0 || x + dx == p[0].len()) {
                return Some(x);
            }
            dx += 1;
        }
    }
    None
}

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let patterns: Vec<Vec<Vec<char>>> = input
            .split("\n\n")
            .map(|b| b.lines().map(|l| l.chars().collect()).collect())
            .collect();

        let mut total = 0;
        for p in patterns {
            if let Some(x) = find(&p, part1) {
                total += x;
            } else if let Some(y) = find(&transpose(&p), part1) {
                total += 100 * y;
            }
        }

        println!("{}", total);
    }
}
