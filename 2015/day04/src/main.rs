fn main() {
    for part1 in [true, false] {
        let input = "iwrupvqb";

        let mut i = 0;
        loop {
            let str = format!("{input}{i}");
            let digest = md5::compute(str.as_bytes());
            let digest = format!("{digest:x}");
            if (part1 && digest.starts_with("00000")) || (!part1 && digest.starts_with("000000")) {
                break;
            }
            i += 1;
        }

        println!("{i}");
    }
}
