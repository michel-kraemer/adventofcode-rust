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
    pub fn insert(&mut self, s: &str) {
        let mut current_node = 0;

        for c in s.chars() {
            debug_assert!(
                c.is_ascii_lowercase(),
                "This trie implementation can only ASCII lowercase characters"
            );

            let i = (c as u8 - b'a') as usize;
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
    pub fn common_prefix_lengths(&self, s: &str) -> Vec<usize> {
        let mut result = Vec::new();
        let mut current_node = 0;

        for (len, c) in s.chars().enumerate() {
            debug_assert!(
                c.is_ascii_lowercase(),
                "This trie implementation can only ASCII lowercase characters"
            );

            if self.nodes[current_node].end {
                result.push(len);
            }

            let i = (c as u8 - b'a') as usize;
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
        t.insert("foo");
        t.insert("foobar");
        t.insert("bar");
        t.insert("bra");
        t.insert("foobarfoo");
        t.insert("fool");
        t.insert("foofoo");
        t.insert("foono");
        t.insert("other");

        assert_eq!(t.common_prefix_lengths("fo"), vec![]);
        assert_eq!(t.common_prefix_lengths("foo"), vec![3]);
        assert_eq!(t.common_prefix_lengths("fool"), vec![3, 4]);
        assert_eq!(t.common_prefix_lengths("foofoo"), vec![3, 6]);
        assert_eq!(t.common_prefix_lengths("foobar"), vec![3, 6]);
        assert_eq!(t.common_prefix_lengths("foobarbar"), vec![3, 6]);
        assert_eq!(t.common_prefix_lengths("foobarfoo"), vec![3, 6, 9]);
        assert_eq!(t.common_prefix_lengths("foobarfool"), vec![3, 6, 9]);
    }
}
