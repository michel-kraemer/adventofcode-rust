#![allow(unused)]

pub fn uints(s: &str) -> Vec<u64> {
    let mut result = Vec::new();
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        while i < bytes.len() && !bytes[i].is_ascii_digit() {
            i += 1;
        }
        if i == bytes.len() {
            break;
        }
        let mut n = 0u64;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            n *= 10;
            n += (bytes[i] - b'0') as u64;
            i += 1;
        }
        result.push(n);
    }
    result
}

pub fn iints(s: &str) -> Vec<i64> {
    let mut result = Vec::new();
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        while i < bytes.len()
            && !bytes[i].is_ascii_digit()
            && !(i < bytes.len() - 1 && bytes[i] == b'-' && bytes[i + 1].is_ascii_digit())
        {
            i += 1;
        }
        if i == bytes.len() {
            break;
        }
        let neg = if bytes[i] == b'-' {
            i += 1;
            true
        } else {
            false
        };
        let mut n = 0i64;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            n *= 10;
            n += (bytes[i] - b'0') as i64;
            i += 1;
        }
        if neg {
            n = -n;
        }
        result.push(n);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uints() {
        assert_eq!(uints(""), vec![]);
        assert_eq!(uints("1 2 3 4 5"), vec![1, 2, 3, 4, 5]);
        assert_eq!(
            uints("one 1 two 2 three 3 four 4 5 6 end"),
            vec![1, 2, 3, 4, 5, 6]
        );
        assert_eq!(uints("Range: 10-42"), vec![10, 42]);
        assert_eq!(
            uints("Ranges: 10-42, 15-16, 0-8, 99-110"),
            vec![10, 42, 15, 16, 0, 8, 99, 110]
        );
        assert_eq!(
            uints("Ranges: 10-42, 15-16, 0-8, 99-110 (inclusive)"),
            vec![10, 42, 15, 16, 0, 8, 99, 110]
        );
        assert_eq!(uints("Button A: X+95, Y+110"), vec![95, 110]);
        assert_eq!(uints("----10---"), vec![10]);
    }

    #[test]
    fn test_iints() {
        assert_eq!(iints(""), vec![]);
        assert_eq!(iints("-1"), vec![-1]);
        assert_eq!(iints("1 -2 3 -4 5"), vec![1, -2, 3, -4, 5]);
        assert_eq!(
            iints("one -1 two 2 three -3 four 4 -5 -6 end"),
            vec![-1, 2, -3, 4, -5, -6]
        );
        assert_eq!(iints("Two numbers: +10-42"), vec![10, -42]);
        assert_eq!(
            iints("Numbers: +10-42, +15-16, +0-8, -99+110"),
            vec![10, -42, 15, -16, 0, -8, -99, 110]
        );
        assert_eq!(
            iints("Numbers: +10-42, +15-16, +0-8, -99+110 (inclusive)"),
            vec![10, -42, 15, -16, 0, -8, -99, 110]
        );
        assert_eq!(iints("Button A: X+95, Y+110"), vec![95, 110]);
        assert_eq!(iints("Trailing minus: X-95, Y+110-"), vec![-95, 110]);
        assert_eq!(iints("Trailing minuses: X-95, Y+110--"), vec![-95, 110]);
        assert_eq!(iints("----10---"), vec![-10]);
        assert_eq!(iints("a 1 b -0 negative zero"), vec![1, 0]);
    }
}
