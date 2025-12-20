fn is_valid(password: &str) -> bool {
    let cs = password.as_bytes();

    if cs.iter().any(|c| *c == b'i' || *c == b'o' || *c == b'l') {
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

    for c in cs.windows(3) {
        if c[1] == c[0] + 1 && c[2] == c[0] + 2 {
            return true;
        }
    }

    false
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
    println!("{n1}");
    let n2 = next(n1);
    println!("{n2}");
}
