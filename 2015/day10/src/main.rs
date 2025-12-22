use std::fs;

fn look_and_say(s: Vec<u8>) -> Vec<u8> {
    let mut result = Vec::new();

    let mut i = 0;
    while i < s.len() {
        let mut n = 1;
        let c = s[i];
        while i < s.len() - 1 && s[i + 1] == c {
            i += 1;
            n += 1;
        }
        result.push(n);
        result.push(c);
        i += 1;
    }

    result
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let mut bytes = input.trim().bytes().map(|b| b - b'0').collect::<Vec<_>>();

    // part 1
    for _ in 0..40 {
        bytes = look_and_say(bytes);
    }
    println!("{}", bytes.len());

    // part 2
    for _ in 0..10 {
        bytes = look_and_say(bytes);
    }
    println!("{}", bytes.len());
}
