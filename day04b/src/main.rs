use std::{fs::File, io::{BufReader, BufRead}, collections::VecDeque, cmp::min};

fn main() {
    let f = File::open("input.txt").expect("Could not open file");
    let reader = BufReader::new(f);

    let lines: Vec<_> = reader.lines().map(|l| l.unwrap()).collect();
    let mut sum = lines.len();
    let mut queue: VecDeque<_> = lines.iter().enumerate().collect();
    while !queue.is_empty() {
        let (i, line) = queue.pop_front().unwrap();
        let parts: Vec<_> = line.split(&[':', '|']).collect();

        let numbers_on_card = parts[1].split(" ").filter(|x| !x.is_empty())
            .map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
        let numbers_i_have = parts[2].split(" ").filter(|x| !x.is_empty())
            .map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
        
        let mut matches = 0;
        for c in numbers_on_card {
            if numbers_i_have.contains(&c) {
                matches += 1;
            }
        }

        let start = i + 1;
        let end = min(lines.len(), start + matches);
        for j in start..end {
            sum += 1;
            queue.push_back((j, &lines[j]));
        }
    }

    println!("Sum: {}", sum);
}
