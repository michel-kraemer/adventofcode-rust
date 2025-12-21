use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines = input.lines().map(|l| l.as_bytes());

    let mut sum1 = 0;
    let mut sum2 = 0;
    for l in lines {
        // decode
        let mut sl = 0;
        let mut i = 1;
        while i < l.len() - 1 {
            let c = l[i];
            if c == b'\\' {
                i += 1;
                let c2 = l[i];
                if c2 == b'\\' || c2 == b'"' {
                    sl += 1;
                } else if c2 == b'x' {
                    sl += 1;
                    i += 2;
                }
            } else {
                sl += 1;
            }
            i += 1;
        }
        sum1 += l.len() - sl;

        // encode
        let mut sl = 2;
        let mut i = 0;
        while i < l.len() {
            let c = l[i];
            if c == b'"' || c == b'\\' {
                sl += 2;
            } else {
                sl += 1;
            }
            i += 1;
        }
        sum2 += sl - l.len();
    }

    println!("{sum1}");
    println!("{sum2}");
}
