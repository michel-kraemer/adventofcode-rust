use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let pixels = input.trim().chars().collect::<Vec<_>>();

    let w = 25;
    let h = 6;

    let mut image = vec![vec![' '; w]; h];

    let mut i = 0;
    let mut min_zeros = usize::MAX;
    let mut min_ones = usize::MAX;
    let mut min_twos = usize::MAX;
    while i < pixels.len() {
        let mut zeros = 0;
        let mut ones = 0;
        let mut twos = 0;

        #[allow(clippy::needless_range_loop)]
        for y in 0..h {
            for x in 0..w {
                let j = i + y * w + x;
                if pixels[j] == '0' {
                    zeros += 1;
                } else if pixels[j] == '1' {
                    ones += 1;
                } else if pixels[j] == '2' {
                    twos += 1;
                }
                if image[y][x] == ' ' && pixels[j] != '2' {
                    if pixels[j] == '1' {
                        image[y][x] = '█';
                    } else {
                        image[y][x] = '.';
                    }
                }
            }
        }

        if zeros < min_zeros {
            min_zeros = zeros;
            min_ones = ones;
            min_twos = twos;
        }

        i += w * h;
    }

    // part 1
    println!("{}", min_ones * min_twos);

    // part 2
    image
        .iter()
        .for_each(|r| println!("{}", r.iter().collect::<String>()));
}
