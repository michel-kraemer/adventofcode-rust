use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let coords = input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(", ").unwrap();
            (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
        })
        .collect::<Vec<_>>();

    let min_x = coords.iter().map(|c| c.0).min().unwrap();
    let min_y = coords.iter().map(|c| c.1).min().unwrap();
    let max_x = coords.iter().map(|c| c.0).max().unwrap();
    let max_y = coords.iter().map(|c| c.1).max().unwrap();

    let mut sizes: HashMap<usize, usize> = HashMap::new();
    let mut bordered = HashSet::new();
    let mut inside = 0;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let distances = coords
                .iter()
                .enumerate()
                .map(|(i, c)| (i, (c.0 - x).abs() + (c.1 - y).abs()))
                .collect::<Vec<_>>();

            let min = distances.iter().min_by_key(|d| d.1).unwrap();
            let count_min = distances.iter().filter(|&d| d.1 == min.1).count();

            if count_min == 1 {
                *sizes.entry(min.0).or_default() += 1;
                if y == min_y || y == max_y || x == min_x || x == max_x {
                    bordered.insert(min.0);
                }
            }

            let total = distances.iter().map(|d| d.1).sum::<i32>();
            if total < 10000 {
                inside += 1;
            }
        }
    }

    // part 1
    sizes.retain(|k, _| !bordered.contains(k));
    let max = sizes.iter().max_by_key(|s| s.1).unwrap();
    println!("{}", max.1);

    // part 2
    println!("{}", inside);
}
