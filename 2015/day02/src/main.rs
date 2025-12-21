use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let mut sum = 0;
    let mut ribbon = 0;
    for l in input.lines() {
        let mut parts = l.split("x");
        let p0 = parts.next().unwrap().parse::<usize>().unwrap();
        let p1 = parts.next().unwrap().parse::<usize>().unwrap();
        let p2 = parts.next().unwrap().parse::<usize>().unwrap();

        let s1 = p0 * p1;
        let s2 = p1 * p2;
        let s3 = p2 * p0;
        let m = s1.min(s2.min(s3));
        sum += 2 * s1;
        sum += 2 * s2;
        sum += 2 * s3;
        sum += m;

        let cubic = s1 * p2;
        let perimeter = if p0 > p1 {
            if p0 > p2 {
                2 * p1 + 2 * p2
            } else {
                2 * p0 + 2 * p1
            }
        } else if p1 > p2 {
            2 * p0 + 2 * p2
        } else {
            2 * p0 + 2 * p1
        };

        ribbon += perimeter + cubic;
    }

    println!("{sum}");
    println!("{ribbon}");
}
