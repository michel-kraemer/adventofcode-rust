use std::fs;

fn has_abba(s: &str) -> bool {
    if s.len() < 3 {
        return false;
    }
    let s = s.as_bytes();
    for &[a, b, c, d] in s.array_windows() {
        if a != b && a == d && b == c {
            return true;
        }
    }
    false
}

fn get_abas(s: &str) -> Vec<(u8, u8)> {
    let mut result = Vec::new();
    if s.len() < 2 {
        return result;
    }
    let sb = s.as_bytes();
    for &[a, b, c] in sb.array_windows() {
        if a != b && a == c {
            result.push((a, b));
        }
    }
    result
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let mut result_part1 = 0;
    let mut result_part2 = 0;
    for l in input.lines() {
        let s = l.split(&['[', ']']).collect::<Vec<_>>();

        let mut abas = Vec::new();
        let mut tls_good = false;
        let mut tls_bad = false;
        for p in s.iter().step_by(2) {
            tls_good |= has_abba(p);
            abas.extend(get_abas(p));
        }

        let mut ssl_good = false;
        for p in s.iter().skip(1).step_by(2) {
            tls_bad |= has_abba(p);
            if !ssl_good {
                'outer: for &[a, b, c] in p.as_bytes().array_windows() {
                    for d in &abas {
                        if a == d.1 && b == d.0 && c == d.1 {
                            ssl_good = true;
                            break 'outer;
                        }
                    }
                }
            }
        }

        if tls_good && !tls_bad {
            result_part1 += 1;
        }

        if ssl_good {
            result_part2 += 1;
        }
    }

    println!("{result_part1}");
    println!("{result_part2}");
}
