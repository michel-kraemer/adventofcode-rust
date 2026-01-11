use std::{collections::BTreeMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let mut events = Vec::new();
    let mut rectangles = Vec::new();
    for l in input.lines() {
        let mut parts = l.split_ascii_whitespace();
        let coords = parts.nth(2).unwrap();
        let size = parts.next().unwrap();
        let (x, y) = coords.split_once(',').unwrap();
        let x = x.parse::<i64>().unwrap();
        let y = y[0..y.len() - 1].parse::<i64>().unwrap();
        let (w, h) = size.split_once('x').unwrap();
        let w = w.parse::<i64>().unwrap();
        let h = h.parse::<i64>().unwrap();
        events.push((x, y, h));
        events.push((x + w, y, -h));
        rectangles.push((x, y, w, h));
    }
    events.sort_unstable();

    // part 1 - perform sweep-line algorithm to count number of overlapping grid
    // cells
    let mut col: BTreeMap<i64, i64> = BTreeMap::new();
    let mut last_x = 0;
    let mut overlapping = 0;
    let mut total = 0;
    let mut i = 0;
    while i < events.len() {
        // start of a new range of events for column x
        let x = events[i].0;

        // update total
        total += overlapping * (x - last_x);
        last_x = x;

        // for each event in the current column ...
        let mut j = i;
        while j < events.len() {
            let (nextx, y, h) = events[j];
            if nextx != x {
                break;
            }
            // update column
            if h > 0 {
                *col.entry(y).or_default() += 1;
                let end = col.entry(y + h).or_default();
                *end -= 1;
                if *end == 0 {
                    col.remove(&(y + h));
                }
            } else {
                let start = col.entry(y).or_default();
                *start -= 1;
                if *start == 0 {
                    col.remove(&y);
                }
                *col.entry(y - h).or_default() += 1;
            }
            j += 1;
        }

        // iterate over column, maintain sum, and update number of overlapping
        // grid cells
        overlapping = 0;
        let mut sum = 0;
        let mut last_y = 0;
        for (&y, &v) in col.iter() {
            if sum > 1 {
                overlapping += y - last_y;
            }
            sum += v;
            last_y = y;
        }

        i = j;
    }
    println!("{total}");

    // part 2 - check for overlaps
    for (i, a) in rectangles.iter().enumerate() {
        let mut good = true;
        for b in rectangles.iter().skip(i + 1) {
            if !(b.0 >= a.0 + a.2 || a.0 >= b.0 + b.2 || b.1 >= a.1 + a.3 || a.1 >= b.1 + b.3) {
                // rectangles are not disjoint
                good = false;
                break;
            }
        }
        if good {
            // rectangle a does not overlap any other rectangle
            println!("{}", i + 1);
            break;
        }
    }
}
