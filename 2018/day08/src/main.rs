use std::{fs, str::Bytes};

/// This is much faster than using split_ascii_whitespace() and then parse()
fn parse_number(bytes: &mut Bytes) -> usize {
    let mut r = 0;
    for b in bytes {
        if !b.is_ascii_digit() {
            break;
        }
        r *= 10;
        r += (b - b'0') as usize;
    }
    r
}

fn parse_node(bytes: &mut Bytes, buffer: &mut [usize]) -> (usize, usize) {
    let n_children = parse_number(bytes);
    let n_metadata = parse_number(bytes);

    let mut metadata_sum = 0;
    for i in 0..n_children {
        let (m, s) = parse_node(bytes, &mut buffer[n_children..]);
        buffer[i] = s;
        metadata_sum += m;
    }

    let mut children_sum = 0;
    for _ in 0..n_metadata {
        let n = parse_number(bytes);
        metadata_sum += n;
        if n_children == 0 {
            children_sum += n;
        } else if n > 0 && n <= n_children {
            children_sum += buffer[n - 1];
        }
    }

    (metadata_sum, children_sum)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let mut buffer = vec![0; input.len()]; // performance: avoid repeated allocation
    let (total1, total2) = parse_node(&mut input.bytes(), &mut buffer);
    println!("{total1}");
    println!("{total2}");
}
