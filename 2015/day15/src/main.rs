use std::fs;

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");

        let ingredients = input
            .lines()
            .map(|l| {
                let (_, is) = l.split_once(": ").unwrap();
                is.split(", ")
                    .map(|i| i.split_once(" ").unwrap().1.parse::<i32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let mut max = 0;
        for i in 1..=97 {
            for j in 1..=99 - i {
                for k in 1..=99 - i - j {
                    let l = 100 - i - j - k;

                    let il = ingredients[0].len();
                    let mut s = vec![0; il];
                    for (n, m) in [i, j, k, l].iter().enumerate() {
                        for u in 0..ingredients[n].len() {
                            s[u] += m * ingredients[n][u];
                        }
                    }

                    for u in 0..il {
                        if s[u] < 0 {
                            s[u] = 0;
                        }
                    }

                    if part1 || s[s.len() - 1] == 500 {
                        let r = s.into_iter().take(il - 1).reduce(|a, b| a * b).unwrap();
                        if r > max {
                            max = r;
                        }
                    }
                }
            }
        }

        println!("{}", max);
    }
}
