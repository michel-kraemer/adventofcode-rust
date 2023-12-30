use std::fs;

fn has_abba(s: &str) -> bool {
    if s.len() < 3 {
        return false;
    }
    let s = s.as_bytes();
    for i in 0..s.len() - 3 {
        if s[i] != s[i + 1] && s[i] == s[i + 3] && s[i + 1] == s[i + 2] {
            return true;
        }
    }
    false
}

fn get_abas(s: &str) -> Vec<&str> {
    let mut result = Vec::new();
    if s.len() < 2 {
        return result;
    }
    let sb = s.as_bytes();
    for i in 0..sb.len() - 2 {
        if sb[i] != sb[i + 1] && sb[i] == sb[i + 2] {
            result.push(&s[i..i + 3]);
        }
    }
    result
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let addresses = input
        .lines()
        .map(|l| l.split(&['[', ']']).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut result_part1 = 0;
    let mut result_part2 = 0;
    for a in addresses {
        let mut abas = Vec::new();
        let mut tls_good = false;
        let mut tls_bad = false;
        for p in a.iter().step_by(2) {
            tls_good |= has_abba(p);
            abas.extend(get_abas(p));
        }

        let reverse_abas = abas
            .into_iter()
            .map(|a| {
                String::from_utf8(vec![a.as_bytes()[1], a.as_bytes()[0], a.as_bytes()[1]]).unwrap()
            })
            .collect::<Vec<_>>();

        let mut ssl_good = false;
        for p in a.iter().skip(1).step_by(2) {
            tls_bad |= has_abba(p);
            ssl_good |= reverse_abas.iter().any(|ra| p.contains(ra));
        }

        if tls_good && !tls_bad {
            result_part1 += 1;
        }

        if ssl_good {
            result_part2 += 1;
        }
    }

    println!("{}", result_part1);
    println!("{}", result_part2);
}
