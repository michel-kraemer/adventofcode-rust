use std::fs;

use crate::btree::Btree;

mod btree {
    pub struct Btree<T: Copy> {
        max_leaf_len: usize,
        buffers: Vec<Vec<T>>,
        len: usize,
    }

    impl<T: Copy> Btree<T> {
        /// Create a new B+tree with a maximum leaf size
        pub fn new(max_leaf_len: usize) -> Self {
            Self {
                max_leaf_len,
                buffers: vec![vec![]],
                len: 0,
            }
        }

        /// Return the number of items in the tree
        pub fn len(&self) -> usize {
            self.len
        }

        /// Insert an `element` into the tree at the given `index`
        pub fn insert(&mut self, index: usize, element: T) {
            // find the leaf that covers this index
            let mut j = 0;
            for (k, b) in self.buffers.iter_mut().enumerate() {
                if index <= j + b.len() {
                    // insert the element into the leaf and update the tree's
                    // length
                    b.insert(index - j, element);
                    self.len += 1;

                    // if the leaf has become too large, split it into two
                    if b.len() > self.max_leaf_len {
                        let new_b = b.split_off(b.len() / 2);
                        self.buffers.insert(k + 1, new_b);
                    }

                    break;
                }
                j += b.len();
            }
        }

        /// Get the element at the given `index` or `None` if the tree does not
        /// contain this index
        pub fn get(&self, index: usize) -> Option<T> {
            let mut j = 0;
            for b in &self.buffers {
                if index < j + b.len() {
                    return Some(b[index - j]);
                }
                j += b.len();
            }
            None
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let steps = input.trim().parse::<usize>().unwrap();

    // part 1 - Instead of inserting elements into a Vec, which would have a
    // worst case complexity of O(n²), we use a simple data structure that
    // resembles a b+tree with only one internal node. A maximum leaf size of
    // 128 seems to be a sweet spot.
    let mut btree = Btree::new(128);
    btree.insert(0, 0);

    let mut i = 0;
    for s in 1..=2017 {
        // compute the index at which to insert the next element
        i = (i + steps) % btree.len() + 1;
        btree.insert(i, s);
    }

    // `i` now points to the element we just inserted. Increase `i` and find the
    // element at this index.
    i = (i + 1) % btree.len();
    println!("{}", btree.get(i).unwrap());

    // part 2 - Since no element will ever be inserted *before* 0, we're
    // actually looking for the element at index 1. We can simulate the
    // insertion of 50,000,000 elements and record every instance of an element
    // being inserted after 0.
    let mut result = 0;
    let mut i = 0;
    let mut len = 1;
    while len < 50_000_000 {
        // jump ahead to just before the end of the buffer
        let add = (len - i) / (steps + 1);
        if add > 0 {
            i += (steps + 1) * add;
            len += add;
        }

        // increment once more to wrap around
        i = (i + steps) % len;
        if i == 0 {
            result = len;
        }
        len += 1;
        i += 1;
    }
    println!("{result}");
}
