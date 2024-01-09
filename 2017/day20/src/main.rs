use std::fs;

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let mut particles = input
            .lines()
            .map(|l| {
                l.split(", ")
                    .map(|v| {
                        v[3..v.len() - 1]
                            .split(',')
                            .map(|i| i.parse::<i64>().unwrap())
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let mut last_r = 0;
        let mut steps_same = 0;
        loop {
            for p in &mut particles {
                p[1][0] += p[2][0];
                p[1][1] += p[2][1];
                p[1][2] += p[2][2];
                p[0][0] += p[1][0];
                p[0][1] += p[1][1];
                p[0][2] += p[1][2];
            }

            if !part1 {
                particles.sort_by(|a, b| a[0].cmp(&b[0]));
                let mut i = 0;
                while i < particles.len() {
                    let mut c = 1;
                    while i + c < particles.len() && particles[i][0] == particles[i + c][0] {
                        c += 1;
                    }
                    if c > 1 {
                        while c > 0 {
                            particles.remove(i);
                            c -= 1;
                        }
                    } else {
                        i += 1;
                    }
                }
            }

            let r = if part1 {
                let mut min_d = usize::MAX;
                let mut min_i = usize::MAX;
                for (i, p) in particles.iter().enumerate() {
                    let d = p[0][0].abs() + p[0][1].abs() + p[0][2].abs();
                    if (d as usize) < min_d {
                        min_d = d as usize;
                        min_i = i;
                    }
                }
                min_i
            } else {
                particles.len()
            };

            if r == last_r {
                steps_same += 1;
            } else {
                last_r = r;
                steps_same = 0;
            }

            // we assume that a maximum of 1000 steps are enough to decide
            // whether we found the solution or not
            if steps_same > 1000 {
                break;
            }
        }

        println!("{}", last_r);
    }
}
