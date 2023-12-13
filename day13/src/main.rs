use std::fs;

fn compare(p1: &Vec<char>, p2: &Vec<char>, ignore_smudge: bool) -> (bool, bool) {
    let mut found_smudge = false;
    for i in 0..p1.len() {
        if p1[i] != p2[i] {
            if ignore_smudge && !found_smudge {
                found_smudge = true;
            } else {
                return (false, found_smudge);
            }
        }
    }
    (true, found_smudge)
}

fn transpose(pattern: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_pattern = vec![vec![' '; pattern.len()]; pattern[0].len()];
    for (y, row) in pattern.into_iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            new_pattern[x][y] = *cell;
        }
    }
    new_pattern
}

fn find_reflection(pattern: &Vec<Vec<char>>, part2: bool) -> Option<usize> {
    for i in 0..pattern.len() - 1 {
        let mut pi = i + 1;
        let mut ni = i + 1;
        let mut found_smudge = !part2;
        while pi > 0 && ni < pattern.len() {
            let cr = compare(&pattern[pi - 1], &pattern[ni], !found_smudge);
            if !cr.0 {
                break;
            }
            if cr.1 {
                found_smudge = true;
            }
            pi -= 1;
            ni += 1;
        }
        if (pi == 0 || ni == pattern.len()) && found_smudge {
            return Some(i + 1);
        }
    }
    None
}

fn main() {
    for part2 in [false, true] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let patterns = input.split("\n\n")
            .map(|p| p.trim().split("\n")
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>()
            ).collect::<Vec<_>>();

        let mut sum = 0;
        for pattern in patterns {
            let hori_ref = find_reflection(&pattern, part2);
            let pattern = transpose(pattern);
            let vert_ref = find_reflection(&pattern, part2);

            sum += hori_ref.map(|i| i * 100).unwrap_or(0);
            sum += vert_ref.unwrap_or(0);
        }

        println!("{}", sum);
    }
}
