use std::fs;

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let triangles = input
            .lines()
            .map(|l| {
                l.split(' ')
                    .filter(|i| !i.is_empty())
                    .map(|i| i.parse::<i64>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let triangles = if part1 {
            triangles
        } else {
            let mut col_triangles = Vec::new();
            for ts in triangles.chunks(3) {
                col_triangles.push(vec![ts[0][0], ts[1][0], ts[2][0]]);
                col_triangles.push(vec![ts[0][1], ts[1][1], ts[2][1]]);
                col_triangles.push(vec![ts[0][2], ts[1][2], ts[2][2]]);
            }
            col_triangles
        };

        let good = triangles
            .into_iter()
            .filter(|t| t[0] + t[1] > t[2] && t[0] + t[2] > t[1] && t[1] + t[2] > t[0])
            .count();

        println!("{}", good);
    }
}
