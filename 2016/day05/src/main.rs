use std::{
    fs,
    sync::{
        atomic::{AtomicBool, AtomicI64, Ordering},
        mpsc,
    },
    thread,
};

use md5::Digest;

const BLOCK_SIZE: i64 = 1000;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let input = input.trim();

    let index = AtomicI64::new(0);
    let stop = AtomicBool::new(false);
    let (sender, receiver) = mpsc::channel();

    let mut password1 = String::new();
    let mut password2 = [' '; 8];

    thread::scope(|scope| {
        // start threads that process numbers in blocks and produce digests
        // starting with 5 zeroes
        let n_threads = thread::available_parallelism().unwrap().into();
        (0..n_threads).for_each(|_| {
            scope.spawn(|| {
                let mut str = input.to_string();
                loop {
                    let stop = stop.load(Ordering::Relaxed);
                    if stop {
                        break;
                    }

                    // fetch next block
                    let index = index.fetch_add(BLOCK_SIZE, Ordering::Relaxed);

                    // process next block
                    let mut result = Vec::new();
                    for j in index..index + BLOCK_SIZE {
                        str.truncate(input.len());
                        str.push_str(&format!("{j}"));
                        let digest = md5::compute(str.as_bytes());
                        if digest.0[0] == 0 && digest.0[1] == 0 && (digest.0[2] >> 4) == 0 {
                            // Digest starts with 5 zeroes. Add it to result.
                            result.push((j, digest));
                        }
                    }

                    // send results of this block to main thread
                    sender.send((index, result)).unwrap();
                }
            });
        });

        // wait for blocks from threads
        let mut queue: Vec<(i64, Vec<(i64, Digest)>)> = Vec::new();
        let mut next = 0;
        'outer: for msg in receiver.iter() {
            // restore order
            let j = queue.partition_point(|o| o.0 < msg.0);
            queue.insert(j, msg);

            // remove as many ordered items from the queue as possible
            while !queue.is_empty() && queue[0].0 == next {
                let r = queue.remove(0);

                for (_, d) in r.1 {
                    // get character at 6th position
                    let c = d[2] & 0b1111;

                    if password1.len() < 8 {
                        // add character to password for part 1
                        password1.push_str(&format!("{:x}", c));
                    }

                    if c < 8 && password2[c as usize] == ' ' {
                        // add character to password for part 2
                        let c2 = d[3] >> 4;
                        if c2 < 10 {
                            password2[c as usize] = (c2 + b'0') as char;
                        } else {
                            password2[c as usize] = (c2 - 10 + b'a') as char;
                        }
                    }

                    if password1.len() == 8 && password2.iter().all(|c| *c != ' ') {
                        // stop if we've got both passwords
                        break 'outer;
                    }
                }

                // wait for the next item in order
                next += BLOCK_SIZE;
            }
        }

        // signal all threads to stop
        stop.store(true, Ordering::Relaxed);
    });

    println!("{password1}");
    println!("{}", password2.iter().collect::<String>());
}
