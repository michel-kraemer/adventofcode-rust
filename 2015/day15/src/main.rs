use std::fs;

fn main() {
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

    let mut max1 = 0;
    let mut max2 = 0;
    for i in 1..=97 {
        let si = ingredients[0].iter().map(|v| v * i).collect::<Vec<_>>();
        for j in 1..=98 - i {
            let sj = si
                .iter()
                .zip(ingredients[1].iter())
                .map(|(s, v)| s + v * j)
                .collect::<Vec<_>>();
            for k in 1..=99 - i - j {
                let l = 100 - i - j - k;

                let s = sj
                    .iter()
                    .zip(ingredients[2].iter())
                    .zip(ingredients[3].iter())
                    .map(|((s, v1), v2)| (s + v1 * k + v2 * l).max(0))
                    .collect::<Vec<_>>();

                let r = s.iter().take(s.len() - 1).product::<i32>();
                max1 = max1.max(r);
                if s[s.len() - 1] == 500 {
                    max2 = max2.max(r);
                }
            }
        }
    }

    println!("{max1}");
    println!("{max2}");
}
