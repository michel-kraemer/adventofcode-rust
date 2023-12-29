use std::fs;

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let input = input.trim();

        let mut result = vec![' '; 8];
        let mut i = 0;
        let mut ci = 0;
        while ci < 8 {
            let pw = format!("{}{}", input, i);
            i += 1;
            let digest = md5::compute(pw);
            let hex = format!("{:x}", digest);
            if hex.starts_with("00000") {
                let c = hex.chars().nth(5).unwrap();
                if part1 {
                    result[ci] = c;
                    ci += 1;
                } else if c.is_ascii_digit() {
                    let pos = c.to_digit(10).unwrap();
                    if pos < 8 && result[pos as usize] == ' ' {
                        result[pos as usize] = hex.chars().nth(6).unwrap();
                        ci += 1;
                    }
                }
            }
        }

        println!("{}", String::from_iter(result));
    }
}
