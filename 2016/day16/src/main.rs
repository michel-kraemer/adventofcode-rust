//! Key insights:
//! 1) The generated pattern can be represented as a binary tree. The leaves are
//!    alternately the initial state and the negated reversed initial state. The
//!    nodes are the bits that need to be inserted between the leaves (the left
//!    child is a `0` and the right child a `1`). Consider the example from the
//!    problem statement `10000` with a disk size of 95 bits:
//!    
//!    ```text
//!                                                   0
//!                           0                       |                       1
//!               0           |           1           |           0           |           1
//!         0     |     1     |     0     |     1     |     0     |     1     |     0     |     1
//!    10000 11110 10000 11110 10000 11110 10000 11110 10000 11110 10000 11110 10000 11110 10000 11110
//!    ```
//!
//! 2) The final checksum always has the same number of digits as the initial
//!    state.
//!
//! 3) In order to calculate one character in the checksum, we just need to
//!    count the number of ones in the corresponding range of bits on the disk.
//!    Example: Consider the tree above as well as the checksum `01100` and disk
//!    size 20 from the problem statement. 20 divided by the checksum length 5
//!    equals 4, so each character in the checksum can be computed by counting
//!    the ones in 4 consecutive bits on the disk. The first 20 bits in the tree
//!    above are `1000 0011 1100 1000 0111`. Count the ones in each block of 4
//!    to get `1 2 2 1 3`. Write a `0` for an odd number of ones and a `1` for
//!    an even number of ones. This results in `01100`, which is exactly the
//!    expected checksum.
//!
//! 4) To compute the final checksum, we therefore just need to perform a binary
//!    search for each character to compute the number of ones in the
//!    corresponding range. The binary search is implemented in the
//!    [Disk::count_ones] method.

use std::{fs, ops::Range};

struct Disk {
    /// The length of the initial state in bits
    input_len: u64,

    /// The initial state
    input: u64,

    /// The cached negated reversed initial state
    input_reverse_neg: u64,

    /// The cached number of ones in both the initial state and the negated
    /// reversed initial state
    both_ones: u64,
}

impl Disk {
    /// Count the number of ones in the given `range` starting at the node
    /// covering the given `node_range`. Set `odd` to 1 if the node is a right
    /// child.
    fn count_ones(&self, range: Range<u64>, node_range: Range<u64>, odd: u64) -> u64 {
        if range.start == range.end {
            // the range is empty so there are no ones
            return 0;
        }

        if node_range.end - node_range.start == self.input_len {
            // we're at a leave
            let v = if (range.start / (self.input_len + 1)).is_multiple_of(2) {
                // the leave is at an even position
                self.input
            } else {
                // the leave is at an odd position
                self.input_reverse_neg
            };

            // extract the queried bits from the leave and count the ones
            let a = v & ((1 << (self.input_len - (range.start % (self.input_len + 1)))) - 1);
            return (a >> (self.input_len - (range.end % (self.input_len + 1)))).count_ones()
                as u64;
        }

        if range.start == node_range.start && range.end == node_range.end {
            // shortcut for performance optimization: if the entire range is
            // queried, directly calculate the number of ones in this node
            // instead of traversing all children
            let i = (node_range.end - node_range.start + 1) / (self.input_len + 1) / 2;
            let mut result = self.both_ones * i;
            if i > 1 {
                result += (1 << i.ilog2()) - 1;
            }
            return result + odd;
        }

        // traverse children ...
        let mid = node_range.start + (node_range.end - node_range.start) / 2;
        let left = node_range.start..mid;
        let right = mid + 1..node_range.end;
        if left.contains(&range.start) && left.contains(&(range.end - 1)) {
            // walk left path
            self.count_ones(range.start..range.end, left, 0)
        } else if right.contains(&range.start) && right.contains(&(range.end - 1)) {
            // walk right path
            self.count_ones(range.start..range.end, right, 1)
        } else {
            // query both children and combine the result
            self.count_ones(range.start..left.end, left, 0)
                + odd
                + self.count_ones(right.start..range.end, right, 1)
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let (input, input_len) = input.trim().bytes().fold((0_u64, 0_u64), |prev, b| {
        let mut next = prev.0 << 1;
        if b == b'1' {
            next += 1;
        }
        (next, prev.1 + 1)
    });
    let input_reverse_neg = !input.reverse_bits() >> (64 - input_len);

    let disk = Disk {
        input_len,
        input,
        input_reverse_neg,
        both_ones: (input.count_ones() + input_reverse_neg.count_ones()) as u64,
    };

    for part1 in [true, false] {
        let disk_size = if part1 { 272 } else { 35651584 };

        // the number of bits we need to query to get one character of the final
        // checksum
        let word_len = disk_size / input_len;

        // the range of the tree's root node
        let root_range = 0..(word_len * (input_len + 1) - 1);

        println!(
            "{}",
            (0..input_len)
                .map(|i| {
                    if disk
                        .count_ones(word_len * i..word_len * (i + 1), root_range.clone(), 0)
                        .is_multiple_of(2)
                    {
                        '1'
                    } else {
                        '0'
                    }
                })
                .collect::<String>()
        );
    }
}
