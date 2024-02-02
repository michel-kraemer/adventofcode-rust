use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let masses = input
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let sum = masses.iter().map(|m| m / 3 - 2).sum::<i32>();
    println!("{}", sum);

    let sum2 = masses
        .into_iter()
        .map(|mut m| {
            let mut s = 0;
            while m > 0 {
                m = m / 3 - 2;
                if m > 0 {
                    s += m;
                }
            }
            s
        })
        .sum::<i32>();
    println!("{}", sum2);
}
