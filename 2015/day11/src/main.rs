use std::fs;

fn is_valid(password: &[u8]) -> bool {
    let mut repeats = 0;
    let mut i = 0;
    while i < password.len() - 1 {
        if password[i] == b'i' || password[i] == b'o' || password[i] == b'l' {
            return false;
        }
        if password[i] == password[i + 1] {
            repeats += 1;
            i += 1;
        }
        i += 1;
    }
    if repeats < 2 {
        return false;
    }

    for c in password.windows(3) {
        if c[1] == c[2] + 1 && c[0] == c[2] + 2 {
            return true;
        }
    }

    false
}

fn inc(password: &mut Vec<u8>) {
    let mut i = 0;
    loop {
        if i == password.len() {
            password.push(b'a');
            break;
        }
        password[i] += 1;
        if password[i] > b'z' {
            password[i] = b'a';
            i += 1;
        } else {
            break;
        }
    }
}

fn next(password: Vec<u8>) -> Vec<u8> {
    let mut password = password;
    loop {
        inc(&mut password);
        if is_valid(&password) {
            return password;
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let n1 = next(input.trim().bytes().rev().collect());
    println!(
        "{}",
        n1.iter().rev().map(|b| *b as char).collect::<String>()
    );
    let n2 = next(n1);
    println!(
        "{}",
        n2.iter().rev().map(|b| *b as char).collect::<String>()
    );
}
