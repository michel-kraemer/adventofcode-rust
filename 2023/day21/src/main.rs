use std::{collections::HashMap, fs, hash::Hash};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Address {
    r: i64,
    c: i64,
}

#[derive(Debug)]
struct Message {
    addr: Address,
    points: Vec<(i64, i64)>,
}

struct Node {
    addr: Address,
    positions: Vec<Vec<bool>>,
    count: usize,
}

impl Node {
    fn new(addr: Address, w: usize, h: usize) -> Self {
        Self {
            addr,
            positions: vec![vec![false; w]; h],
            count: 0,
        }
    }

    fn calculate_next_state(&mut self, grid: &[Vec<char>], size: usize) -> Vec<Message> {
        let mut messages = Vec::new();

        let mut new_positions = vec![vec![false; size]; size];
        let mut count = 0;
        for y in 0..size {
            for x in 0..size {
                if !self.positions[y][x] {
                    continue;
                }
                for d in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let nx = x as i64 + d.0;
                    let ny = y as i64 + d.1;
                    if nx < 0 || nx >= size as i64 || ny < 0 || ny >= size as i64 {
                        // point is outside the grid, send message to neighbor
                        messages.push(Message {
                            addr: Address {
                                r: self.addr.r + d.1,
                                c: self.addr.c + d.0,
                            },
                            points: vec![(nx - d.0 * size as i64, ny - d.1 * size as i64)],
                        });
                    } else if grid[ny as usize][nx as usize] != '#'
                        && !new_positions[ny as usize][nx as usize]
                    {
                        count += 1;
                        new_positions[ny as usize][nx as usize] = true;
                    }
                }
            }
        }
        self.positions = new_positions;
        self.count = count;

        messages
    }

    fn process_message(&mut self, msg: Message) {
        for (nx, ny) in msg.points {
            if !self.positions[ny as usize][nx as usize] {
                self.count += 1;
                self.positions[ny as usize][nx as usize] = true;
            }
        }
    }
}

fn main() {
    // this is a generalized solution that will be a bit slower than most
    // solutions you find on the Internet, but it should work for every input
    // and every step count

    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let grid = input
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        assert_eq!(grid[0].len(), grid.len(), "Grid must be square");
        let size = grid.len();

        // find start position
        let mut start = (0i64, 0i64);
        for (y, row) in grid.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == 'S' {
                    start = (x as i64, y as i64);
                }
            }
        }

        // prepare center node
        let mut center_positions = vec![vec![false; size]; size];
        center_positions[start.1 as usize][start.0 as usize] = true;
        let mut nodes = HashMap::new();
        let addr = Address { r: 0, c: 0 };
        nodes.insert(
            addr,
            Node {
                addr,
                positions: center_positions,
                count: 1,
            },
        );

        let max_steps = if part1 { 64 } else { 26501365 };

        // Break at a point where we can extrapolate the final count. We
        // basically try to proceed far enough until we can get at least three
        // rows above and below the center row. Then we proceed
        // (max_steps % size) additional steps, to get to a state that
        // can be extrapolated.
        let break_steps = 3 * size + max_steps % size - 1;

        let mut steps = 0;
        while steps < max_steps {
            // calculate next state for each node and record messages they
            // send to their neighbors
            let mut messages = Vec::new();
            for n in nodes.values_mut() {
                messages.extend(n.calculate_next_state(&grid, size));
            }

            // process messages
            for msg in messages {
                let recipient = nodes
                    .entry(msg.addr)
                    .or_insert_with(|| Node::new(msg.addr, size, size));
                recipient.process_message(msg);
            }

            // break if we reached the break point
            if steps == break_steps {
                break;
            }
            steps += 1;
        }

        if part1 || steps != break_steps {
            println!("{}", nodes.values().map(|c| c.count).sum::<usize>());
        } else {
            let min_row = nodes.values().map(|n| n.addr.r).min().unwrap();
            let max_row = nodes.values().map(|n| n.addr.r).max().unwrap();
            let mut row_sums: HashMap<i64, usize> = HashMap::new();
            let mut row_counts: HashMap<i64, usize> = HashMap::new();
            for n in nodes.values() {
                *row_sums.entry(n.addr.r).or_default() += n.count;
                *row_counts.entry(n.addr.r).or_default() += 1;
            }

            // the increment per additional row (i.e. in each additional
            // extrapolated row, the count of the center node plus its neighbor
            // must be added to the previous row)
            let inc_per_row =
                nodes[&Address { r: 0, c: 0 }].count + nodes[&Address { r: 0, c: 1 }].count;

            // calculate the virtual number of rows above the center row
            let max_rows = max_steps as f64 / size as f64;
            let max_rows = if row_counts[&min_row] == 1 {
                max_rows.ceil() as i64
            } else {
                max_rows.floor() as i64
            };

            // calculate the virtual number of columns to the left and right
            // of the center column
            let max_cols = (max_steps as f64 / size as f64).floor() as i64;

            // calculate the total count ...
            // copy the first 3 rows
            let mut total = 0;
            for r in min_row..min_row + 3 {
                total += row_sums[&r];
            }
            // Add more rows as necessary. Copy the value of the third row
            // and add the increment for each additional row.
            for i in 1..=max_rows - 3 {
                total += row_sums[&(min_row + 2)] + inc_per_row * i as usize;
            }
            // copy the center row and add the increment as necessary
            total += row_sums[&0] + inc_per_row * (max_cols - 3) as usize;
            // add the rows below the center row
            for i in 1..=max_rows - 3 {
                total += row_sums[&(max_row - 2)] + inc_per_row * i as usize;
            }
            // copy the last 3 rows
            for r in max_row - 2..=max_row {
                total += row_sums[&r];
            }

            println!("{}", total);
        }
    }
}
