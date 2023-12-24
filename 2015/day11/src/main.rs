fn is_valid(password: &str) -> bool {
    let cs = password.chars().collect::<Vec<_>>();

    if cs.iter().any(|c| *c == 'i' || *c == 'o' || *c == 'l') {
        return false;
    }

    let mut repeats = 0;
    let mut i = 0;
    while i < cs.len() - 1 {
        if cs[i] == cs[i + 1] {
            repeats += 1;
            i += 1;
        }
        i += 1;
    }
    if repeats < 2 {
        return false;
    }

    let mut contains_sequence = false;
    i = 0;
    while i < cs.len() - 2 {
        let c1 = cs[i];
        let c2 = (c1 as u8 + 1) as char;
        let c3 = (c2 as u8 + 1) as char;
        if cs[i + 1] == c2 && cs[i + 2] == c3 {
            contains_sequence = true;
            break;
        }
        i += 1;
    }

    if !contains_sequence {
        return false;
    }

    true
}

fn inc(password: String) -> String {
    // decode
    let mut i: i64 = 0;
    for c in password.chars() {
        i *= 26;
        i += (c as u8 - b'a') as i64;
    }

    i += 1;

    // encode
    let mut result = String::new();
    while i > 0 {
        let m = (i % 26) as u8;
        i /= 26;
        result.insert(0, (m + b'a') as char);
    }

    result
}

fn next(password: String) -> String {
    let mut password = password;
    loop {
        password = inc(password);
        if is_valid(&password) {
            return password;
        }
    }
}

fn main() {
    let input = "hepxcrrq".to_string();
    let n1 = next(input);
    println!("{}", n1);
    let n2 = next(n1);
    println!("{}", n2);
}
