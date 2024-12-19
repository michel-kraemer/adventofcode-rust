//! A prefix tree (trie) that allows us to quickly search for common prefixes.
//!
//! Since the puzzle input only consists of lowercase characters from 'a' to
//! 'z' and the towel patterns are on average rather short, I didn't bother
//! implementing something sophisticated. A naive implementation where the
//! nodes have an array of 26 pointers to their children is enough and does not
//! consume too much memory.

#[derive(Debug, Default)]
struct Node {
    children: [usize; 26],
    end: bool,
}

pub struct Trie {
    nodes: Vec<Node>,
}

impl Trie {
    pub fn default() -> Self {
        // insert root
        Trie {
            nodes: vec![Node::default()],
        }
    }

    /// Insert a prefix into the tree
    pub fn insert(&mut self, s: &[u8]) {
        let mut current_node = 0;

        for c in s {
            debug_assert!(
                c.is_ascii_lowercase(),
                "This trie implementation only supports ASCII lowercase characters"
            );

            let i = (c - b'a') as usize;
            if self.nodes[current_node].children[i] == 0 {
                self.nodes[current_node].children[i] = self.nodes.len();
                self.nodes.push(Node::default());
            }

            current_node = self.nodes[current_node].children[i];
        }

        self.nodes[current_node].end = true;
    }

    /// Look for prefixes of the given string and return their lengths. If
    /// there is no prefix, an empty Vec will be returned.
    pub fn common_prefix_lengths(&self, s: &[u8]) -> Vec<usize> {
        let mut result = Vec::new();
        let mut current_node = 0;

        for (len, c) in s.iter().enumerate() {
            debug_assert!(
                c.is_ascii_lowercase(),
                "This trie implementation only supports ASCII lowercase characters"
            );

            if self.nodes[current_node].end {
                result.push(len);
            }

            let i = (c - b'a') as usize;
            if self.nodes[current_node].children[i] == 0 {
                // nothing else to find
                return result;
            }

            current_node = self.nodes[current_node].children[i];
        }

        if self.nodes[current_node].end {
            result.push(s.len());
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Trie;

    #[test]
    fn common_prefix_lengths() {
        let mut t = Trie::default();
        t.insert(b"foo");
        t.insert(b"foobar");
        t.insert(b"bar");
        t.insert(b"bra");
        t.insert(b"foobarfoo");
        t.insert(b"fool");
        t.insert(b"foofoo");
        t.insert(b"foono");
        t.insert(b"other");

        assert_eq!(t.common_prefix_lengths(b"fo"), vec![]);
        assert_eq!(t.common_prefix_lengths(b"foo"), vec![3]);
        assert_eq!(t.common_prefix_lengths(b"fool"), vec![3, 4]);
        assert_eq!(t.common_prefix_lengths(b"foofoo"), vec![3, 6]);
        assert_eq!(t.common_prefix_lengths(b"foobar"), vec![3, 6]);
        assert_eq!(t.common_prefix_lengths(b"foobarbar"), vec![3, 6]);
        assert_eq!(t.common_prefix_lengths(b"foobarfoo"), vec![3, 6, 9]);
        assert_eq!(t.common_prefix_lengths(b"foobarfool"), vec![3, 6, 9]);
    }
}
