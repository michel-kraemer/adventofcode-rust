use std::{collections::HashMap, fs, hash::Hash};

use bitarray::BitArray;

mod bitarray;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Address {
    r: i64,
    c: i64,
}

enum MessageBody {
    Row(i64, BitArray),
    Point(i64, i64),
}

struct Message {
    addr: Address,
    body: MessageBody,
}

struct Node {
    addr: Address,
    positions: Vec<BitArray>,
}

impl Node {
    fn new(addr: Address, size: usize) -> Self {
        Self {
            addr,
            positions: vec![BitArray::new(size); size],
        }
    }

    fn count(&self) -> usize {
        self.positions.iter().map(|p| p.count_ones()).sum::<usize>()
    }

    fn calculate_next_state(&mut self, grid_bits: &[BitArray], size: usize) -> Vec<Message> {
        let mut messages = Vec::new();

        // Build new_positions by shifting the current positions to the top,
        // left, right, and bottom. Record bits shifted out of the grid and
        // send them to our neighbors.
        let mut new_positions = vec![BitArray::new(size); size];
        for y in 0..size {
            // shift up (copy bits to the row above)
            if y > 0 {
                new_positions[y - 1] |= &self.positions[y];
            } else if self.positions[y].count_ones() > 0 {
                messages.push(Message {
                    addr: Address {
                        r: self.addr.r - 1,
                        c: self.addr.c,
                    },
                    body: MessageBody::Row(size as i64 - 1, self.positions[y].clone()),
                });
            }

            // shift down (copy bits to the row below)
            if y < size - 1 {
                new_positions[y + 1] |= &self.positions[y];
            } else if self.positions[y].count_ones() > 0 {
                messages.push(Message {
                    addr: Address {
                        r: self.addr.r + 1,
                        c: self.addr.c,
                    },
                    body: MessageBody::Row(0, self.positions[y].clone()),
                });
            }

            // shift left
            self.positions[y].rotate_left();
            let mut reset_left = false;
            if self.positions[y].get(size - 1) {
                messages.push(Message {
                    addr: Address {
                        r: self.addr.r,
                        c: self.addr.c - 1,
                    },
                    body: MessageBody::Point(size as i64 - 1, y as i64),
                });
                reset_left = true;
                self.positions[y].clear(size - 1);
            }

            new_positions[y] |= &self.positions[y];

            if reset_left {
                self.positions[y].set(size - 1);
            }

            // shift right
            self.positions[y].rotate_right();
            self.positions[y].rotate_right();
            if self.positions[y].get(0) {
                messages.push(Message {
                    addr: Address {
                        r: self.addr.r,
                        c: self.addr.c + 1,
                    },
                    body: MessageBody::Point(0, y as i64),
                });
                self.positions[y].clear(0);
            }
            new_positions[y] |= &self.positions[y];

            // clear bits where there are rocks in the grid
            if y > 0 {
                new_positions[y - 1].clear_all(&grid_bits[y - 1]);
            }
        }
        new_positions[size - 1].clear_all(&grid_bits[size - 1]);

        self.positions = new_positions;

        messages
    }

    fn process_message(&mut self, msg: Message) {
        match msg.body {
            MessageBody::Row(y, row) => {
                self.positions[y as usize] |= &row;
            }
            MessageBody::Point(x, y) => {
                self.positions[y as usize].set(x as usize);
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
        assert!(size % 2 == 1, "Grid size must be odd");

        // find start position
        let mut start = (0i64, 0i64);
        for (y, row) in grid.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == 'S' {
                    start = (x as i64, y as i64);
                }
            }
        }
        assert!(
            start.0 == size as i64 / 2 && start.1 == size as i64 / 2,
            "Start position must be in the center"
        );

        // convert grid to bit array
        let mut grid_bits = vec![BitArray::new(size); size];
        for (y, row) in grid.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == '#' {
                    grid_bits[y].set(x);
                }
            }
        }

        // prepare center node
        let mut center_positions = vec![BitArray::new(size); size];
        center_positions[start.1 as usize].set(start.0 as usize);
        let mut nodes = HashMap::new();
        let addr = Address { r: 0, c: 0 };
        nodes.insert(
            addr,
            Node {
                addr,
                positions: center_positions,
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
                messages.extend(n.calculate_next_state(&grid_bits, size));
            }

            // process messages
            for msg in messages {
                let recipient = nodes
                    .entry(msg.addr)
                    .or_insert_with(|| Node::new(msg.addr, size));
                recipient.process_message(msg);
            }

            // break if we reached the break point
            if steps == break_steps {
                break;
            }
            steps += 1;
        }

        if part1 || steps != break_steps {
            println!("{}", nodes.values().map(|c| c.count()).sum::<usize>());
        } else {
            let min_row = nodes.values().map(|n| n.addr.r).min().unwrap();
            let max_row = nodes.values().map(|n| n.addr.r).max().unwrap();
            let mut row_sums: HashMap<i64, usize> = HashMap::new();
            let mut row_counts: HashMap<i64, usize> = HashMap::new();
            for n in nodes.values() {
                *row_sums.entry(n.addr.r).or_default() += n.count();
                *row_counts.entry(n.addr.r).or_default() += 1;
            }

            // the increment per additional row (i.e. in each additional
            // extrapolated row, the count of the center node plus its neighbor
            // must be added to the previous row)
            let inc_per_row =
                nodes[&Address { r: 0, c: 0 }].count() + nodes[&Address { r: 0, c: 1 }].count();

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
