use std::fs;

fn has_abba(s: &str) -> bool {
    if s.len() < 3 {
        return false;
    }
    let s = s.as_bytes();
    for w in s.windows(4) {
        if w[0] != w[1] && w[0] == w[3] && w[1] == w[2] {
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
    for w in sb.windows(3) {
        if w[0] != w[1] && w[0] == w[2] {
            result.push((w[0], w[1]));
        }
    }
    result
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let mut result_part1 = 0;
    let mut result_part2 = 0;
    for l in input.lines() {
        let a = l.split(&['[', ']']).collect::<Vec<_>>();

        let mut abas = Vec::new();
        let mut tls_good = false;
        let mut tls_bad = false;
        for p in a.iter().step_by(2) {
            tls_good |= has_abba(p);
            abas.extend(get_abas(p));
        }

        let mut ssl_good = false;
        for p in a.iter().skip(1).step_by(2) {
            tls_bad |= has_abba(p);
            if !ssl_good {
                'outer: for w in p.as_bytes().windows(3) {
                    for a in &abas {
                        if w[0] == a.1 && w[1] == a.0 && w[2] == a.1 {
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
