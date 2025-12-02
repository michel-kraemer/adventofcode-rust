use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    for part in [1, 2] {
        let mut total = 0;
        for range in input.trim().split(",") {
            let (lo, hi) = range.split_once("-").unwrap();
            let lo = lo.parse::<i64>().unwrap();
            let hi = hi.parse::<i64>().unwrap();
            for n in 10.max(lo)..=hi {
                let s = n.to_string().bytes().collect::<Vec<_>>();
                if part == 1 {
                    if s.len().is_multiple_of(2) && s[..s.len() / 2] == s[s.len() / 2..] {
                        total += n;
                    }
                } else {
                    for chunklen in 1..=s.len() / 2 {
                        let mut c = s.chunks(chunklen);
                        let first = c.next().unwrap();
                        let mut good = true;
                        for next in c {
                            if next != first {
                                good = false;
                                break;
                            }
                        }
                        if good {
                            total += n;
                            break;
                        }
                    }
                }
            }
        }
        println!("{total}");
    }
}
