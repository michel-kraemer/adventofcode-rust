use std::{f32::consts::PI, fs};

fn get_sum(prefix_sums: &[i32], x: usize, y: usize, s: usize, w: usize) -> i32 {
    let x = x - 1;
    let y = y - 1;
    prefix_sums[(y + s) * w + (x + s)] - prefix_sums[y * w + (x + s)] - prefix_sums[(y + s) * w + x]
        + prefix_sums[y * w + x]
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Could not read file")
        .trim()
        .parse::<i32>()
        .unwrap();

    const W: usize = 300;

    let mut grid = [0i32; (W + 1) * (W + 1)];
    for y in 1..=W {
        for x in 1..=W {
            let rack_id = (x as i32) + 10;
            let power = ((rack_id * (y as i32) + input) * rack_id) / 100 % 10 - 5;
            grid[y * W + x] = power;
        }
    }

    // compute summed area table
    let mut prefix_sums = [0i32; (W + 1) * (W + 1)];
    for y in 1..=W {
        for x in 1..=W {
            prefix_sums[y * W + x] =
                grid[y * W + x] + prefix_sums[(y - 1) * W + x] + prefix_sums[y * W + x - 1]
                    - prefix_sums[(y - 1) * W + x - 1];
        }
    }

    // part 1
    let mut max1 = 0;
    let mut max_coords1 = (0, 0);
    for y in 1..=W - 2 {
        for x in 1..=W - 2 {
            let sum = get_sum(&prefix_sums, x, y, 3, W);
            if sum > max1 {
                max1 = sum;
                max_coords1 = (x, y);
            }
        }
    }
    println!("{},{}", max_coords1.0, max_coords1.1);

    // part 2 - use summed area table (just like in part 1) but parallelize
    // across multiple threads

    // Determine which thread processes how many square sizes. The results for
    // larger squares (across the grid) can be calculated faster so we dedicate
    // fewer threads to them.
    let threads = std::thread::available_parallelism().unwrap().get();
    let mut prev = 0.0;
    let mut total_sum = 0;
    let mut block_lens = Vec::new();
    for i in 1..=threads {
        let next = (PI / (2.0 * threads as f32) * i as f32).sin();
        let diff = next - prev;
        prev = next;
        let s = (diff * W as f32).round() as usize;
        total_sum += s;
        block_lens.push(s);
    }
    if total_sum < W {
        block_lens[0] += W - total_sum;
    }
    if total_sum > W {
        block_lens[0] -= total_sum - W;
    }

    let mut max2 = 0;
    let mut max_coords2 = (0, 0);
    let mut max_size2 = 0;
    std::thread::scope(|scope| {
        let mut handles = Vec::new();

        total_sum = 0;
        for bl in block_lens.into_iter().rev() {
            let range = total_sum..total_sum + bl;
            total_sum += bl;
            handles.push(scope.spawn(|| {
                let mut thread_max = 0;
                let mut thread_max_coords = (0, 0);
                let mut thread_max_size = 0;
                for s in range {
                    for y in 1..=W - s + 1 {
                        for x in 1..=W - s + 1 {
                            let sum = get_sum(&prefix_sums, x, y, s, W);
                            if sum > thread_max {
                                thread_max = sum;
                                thread_max_coords = (x, y);
                                thread_max_size = s;
                            }
                        }
                    }
                }
                (thread_max, thread_max_coords, thread_max_size)
            }));
        }

        for h in handles {
            let (tm, tmc, tms) = h.join().unwrap();
            if tm > max2 {
                max2 = tm;
                max_coords2 = tmc;
                max_size2 = tms;
            }
        }
    });

    println!("{},{},{max_size2}", max_coords2.0, max_coords2.1);
}
