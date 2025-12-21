use std::{
    fs,
    sync::atomic::{AtomicI64, Ordering},
    thread,
};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let input = input.trim();
    let i = AtomicI64::new(0);
    let min1 = AtomicI64::new(i64::MAX);
    let min2 = AtomicI64::new(i64::MAX);

    thread::scope(|scope| {
        let n_threads = thread::available_parallelism().unwrap().into();
        (0..n_threads).for_each(|_| {
            scope.spawn(|| {
                let mut str = input.to_string();
                loop {
                    let i = i.fetch_add(1, Ordering::Relaxed);
                    if min1.load(Ordering::Relaxed) < i && min2.load(Ordering::Relaxed) < i {
                        break;
                    }
                    str.truncate(input.len());
                    str.push_str(&format!("{i}"));
                    let digest = md5::compute(str.as_bytes());
                    if digest.0[0] == 0 && digest.0[1] == 0 {
                        if digest.0[2] == 0 {
                            min2.fetch_min(i, Ordering::Relaxed);
                        } else if (digest.0[2] >> 4) == 0 {
                            min1.fetch_min(i, Ordering::Relaxed);
                        }
                    }
                }
            });
        });
    });

    println!("{}", min1.load(Ordering::Relaxed));
    println!("{}", min2.load(Ordering::Relaxed));
}
