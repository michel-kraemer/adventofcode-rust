use std::fs;

use num::integer::lcm;

fn calculate_energy(moons: &[[i32; 3]], steps: usize) -> u64 {
    let mut moons = moons.to_owned();
    let mut velocities = vec![[0i32; 3]; moons.len()];

    for _ in 0..steps {
        for i in 0..moons.len() {
            for j in i + 1..moons.len() {
                for a in 0..3 {
                    if moons[i][a] < moons[j][a] {
                        velocities[i][a] += 1;
                        velocities[j][a] -= 1;
                    }
                    if moons[i][a] > moons[j][a] {
                        velocities[i][a] -= 1;
                        velocities[j][a] += 1;
                    }
                }
            }
        }

        for i in 0..moons.len() {
            for a in 0..3 {
                moons[i][a] += velocities[i][a];
            }
        }
    }

    let mut energy = 0;
    for i in 0..moons.len() {
        let pot = moons[i].iter().map(|p| p.abs()).sum::<i32>() as u64;
        let kin = velocities[i].iter().map(|v| v.abs()).sum::<i32>() as u64;
        energy += pot * kin;
    }

    energy
}

fn find_cycle(mut moons: Vec<i32>) -> usize {
    let initial_moons = moons.clone();
    let mut velocities = vec![0; moons.len()];

    let mut steps = 0;
    loop {
        for i in 0..moons.len() {
            for j in i + 1..moons.len() {
                if moons[i] < moons[j] {
                    velocities[i] += 1;
                    velocities[j] -= 1;
                }
                if moons[i] > moons[j] {
                    velocities[i] -= 1;
                    velocities[j] += 1;
                }
            }
        }

        for i in 0..moons.len() {
            moons[i] += velocities[i];
        }

        steps += 1;

        if moons == initial_moons && velocities.iter().all(|&v| v == 0) {
            // found cycle
            return steps;
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let moons = input
        .lines()
        .map(|l| {
            let p = l.split(&['=', ',', '>']).collect::<Vec<_>>();
            [
                p[1].parse::<i32>().unwrap(),
                p[3].parse::<i32>().unwrap(),
                p[5].parse::<i32>().unwrap(),
            ]
        })
        .collect::<Vec<_>>();

    // part 1
    println!("{}", calculate_energy(&moons, 1000));

    // part 2
    let cx = find_cycle(moons.iter().map(|m| m[0]).collect());
    let cy = find_cycle(moons.iter().map(|m| m[1]).collect());
    let cz = find_cycle(moons.iter().map(|m| m[2]).collect());

    println!("{}", lcm(cx, lcm(cy, cz)));
}
