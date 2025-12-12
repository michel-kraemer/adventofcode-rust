use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let blocks = input.split("\n\n").collect::<Vec<_>>();
    let presents = &blocks[0..blocks.len() - 1];
    let areas = blocks[blocks.len() - 1];

    let num_parts_per_present = presents
        .iter()
        .map(|p| p.as_bytes().iter().copied().filter(|b| *b == b'#').count())
        .collect::<Vec<_>>();

    let mut total = 0;
    for a in areas.lines() {
        let parts = a.split_ascii_whitespace().collect::<Vec<_>>();
        let (width, height) = parts[0].split_once('x').unwrap();
        let width = width.parse::<usize>().unwrap();
        let height = height[0..height.len() - 1].parse::<usize>().unwrap();

        let required_presents = parts[1..]
            .iter()
            .map(|p| p.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let area = width * height;
        let mut required_area = 0;
        for (i, t) in required_presents.iter().enumerate() {
            required_area += t * num_parts_per_present[i];
        }
        if required_area <= area {
            total += 1;
        }
    }
    println!("{total}");
}
