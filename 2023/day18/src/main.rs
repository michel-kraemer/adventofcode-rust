use std::fs;

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let lines = input.lines().collect::<Vec<_>>();

        let mut pos = (0i64, 0i64);
        let mut vertices = vec![pos];
        let mut countv = 0;

        for l in lines {
            let sp = l.split_whitespace().collect::<Vec<_>>();
            let (dir, steps) = if part1 {
                (sp[0], sp[1].parse::<i64>().unwrap())
            } else {
                let steps = i64::from_str_radix(&sp[2][2..7], 16).unwrap();
                (&sp[2][7..8], steps)
            };

            countv += steps;

            match dir {
                "0" | "R" => pos.0 += steps,
                "1" | "D" => pos.1 += steps,
                "2" | "L" => pos.0 -= steps,
                "3" | "U" => pos.1 -= steps,
                _ => panic!("Unknown direction"),
            }

            vertices.push(pos);
        }

        // calculate polygon area
        let mut sum = 0;
        for i in 0..vertices.len() - 1 {
            sum += (vertices[i].1 + vertices[i + 1].1) * (vertices[i].0 - vertices[i + 1].0);
        }
        sum /= 2;

        // count number of interior points using Pick's theorem
        let i = sum.abs() - countv / 2 + 1;

        // sum interior points and vertices
        println!("{}", i + countv);
    }
}
