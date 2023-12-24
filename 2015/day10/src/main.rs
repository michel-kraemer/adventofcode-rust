fn look_and_say(s: String) -> String {
    let mut result = String::new();

    let s = s.chars().collect::<Vec<_>>();
    let mut i = 0;
    while i < s.len() {
        let mut n = 1;
        let c = s[i];
        while i < s.len() - 1 && s[i + 1] == c {
            i += 1;
            n += 1;
        }
        result.push_str(&format!("{n}"));
        result.push(c);
        i += 1;
    }

    result
}

fn main() {
    for part1 in [true, false] {
        let mut input = "3113322113".to_string();
        for _ in 0..(if part1 { 40 } else { 50 }) {
            input = look_and_say(input);
        }
        println!("{}", input.len());
    }
}
