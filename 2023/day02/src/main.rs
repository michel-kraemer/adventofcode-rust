use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();

    let mut sum = 0;
    let mut power = 0;
    for (game, l) in lines.into_iter().enumerate() {
        let (_, sets) = l.split_once(":").unwrap();
        let draws = sets.split(";");
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for d in draws {
            let cubes = d.split(",");
            for c in cubes {
                let mut ps = c.split_whitespace();
                let n = ps.next().unwrap().parse::<u32>().unwrap();
                let color = ps.next().unwrap();
                match color {
                    "red" => max_red = max_red.max(n),
                    "green" => max_green = max_green.max(n),
                    "blue" => max_blue = max_blue.max(n),
                    _ => unreachable!(),
                }
            }
        }
        if max_red <= 12 && max_green <= 13 && max_blue <= 14 {
            sum += game + 1;
        }
        power += max_red * max_green * max_blue;
    }

    println!("{}", sum);
    println!("{}", power);
}
