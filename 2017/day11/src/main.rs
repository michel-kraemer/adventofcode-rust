use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let steps = input.trim().split(',');

    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;

    let mut max = 0;
    for s in steps {
        match s {
            "n" => {
                j += 1;
                k -= 1;
            }
            "ne" => {
                i += 1;
                k -= 1;
            }
            "se" => {
                i += 1;
                j -= 1;
            }
            "s" => {
                k += 1;
                j -= 1;
            }
            "sw" => {
                i -= 1;
                k += 1;
            }
            "nw" => {
                i -= 1;
                j += 1;
            }
            _ => panic!(),
        }
        let d = (i.abs() + j.abs() + k.abs()) / 2;
        max = max.max(d);
    }

    println!("{}", (i.abs() + j.abs() + k.abs()) / 2);
    println!("{}", max);
}
