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

pub struct PrefixLengthIterator<'s, 't> {
    /// The string to iterate over
    s: &'s [u8],

    /// The index in `s`
    i: usize,

    /// The Trie containing the prefixes
    trie: &'t Trie,

    /// The current node in the Trie
    current_node: usize,
}

impl<'s, 't> Iterator for PrefixLengthIterator<'s, 't> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.i < self.s.len() {
            let c = self.s[self.i];
            debug_assert!(
                c.is_ascii_lowercase(),
                "This trie implementation only supports ASCII lowercase characters"
            );

            self.i += 1;

            self.current_node = self.trie.nodes[self.current_node].children[(c - b'a') as usize];
            if self.current_node == 0 {
                // nothing else to find
                return None;
            } else if self.trie.nodes[self.current_node].end {
                return Some(self.i);
            }
        }
        None
    }
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

    /// Look for prefixes of the given string and return an iterator over
    /// their lengths
    pub fn common_prefix_lengths<'s, 't>(&'t self, s: &'s [u8]) -> PrefixLengthIterator<'s, 't> {
        PrefixLengthIterator {
            s,
            i: 0,
            trie: self,
            current_node: 0,
        }
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

        assert_eq!(t.common_prefix_lengths(b"fo").collect::<Vec<_>>(), vec![]);
        assert_eq!(t.common_prefix_lengths(b"foo").collect::<Vec<_>>(), vec![3]);
        assert_eq!(
            t.common_prefix_lengths(b"fool").collect::<Vec<_>>(),
            vec![3, 4]
        );
        assert_eq!(
            t.common_prefix_lengths(b"foofoo").collect::<Vec<_>>(),
            vec![3, 6]
        );
        assert_eq!(
            t.common_prefix_lengths(b"foobar").collect::<Vec<_>>(),
            vec![3, 6]
        );
        assert_eq!(
            t.common_prefix_lengths(b"foobarbar").collect::<Vec<_>>(),
            vec![3, 6]
        );
        assert_eq!(
            t.common_prefix_lengths(b"foobarfoo").collect::<Vec<_>>(),
            vec![3, 6, 9]
        );
        assert_eq!(
            t.common_prefix_lengths(b"foobarfool").collect::<Vec<_>>(),
            vec![3, 6, 9]
        );
    }
}
