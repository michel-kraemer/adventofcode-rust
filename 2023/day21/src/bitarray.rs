use std::ops::BitOrAssign;

#[derive(Clone)]
pub struct BitArray {
    data: Vec<u64>,
    size: usize,
}

impl BitArray {
    pub fn new(size: usize) -> Self {
        let data = vec![0; (size + 63) / 64];
        Self { data, size }
    }

    pub fn count_ones(&self) -> usize {
        self.data.iter().map(|x| x.count_ones() as usize).sum()
    }

    pub fn set(&mut self, index: usize) {
        self.data[index / 64] |= 1 << (63 - index % 64);
    }

    pub fn get(&self, index: usize) -> bool {
        self.data[index / 64] & (1 << (63 - index % 64)) != 0
    }

    pub fn clear(&mut self, index: usize) {
        self.data[index / 64] &= !(1 << (63 - index % 64));
    }

    pub fn clear_all(&mut self, other: &BitArray) {
        for i in 0..self.data.len() {
            self.data[i] &= !other.data[i];
        }
    }

    pub fn rotate_left(&mut self) {
        let l = self.data.len();
        let mut carry = 0;
        for i in (0..l).rev() {
            let new_carry = self.data[i] >> 63;
            self.data[i] = (self.data[i] << 1) | carry;
            carry = new_carry;
        }
        self.data[l - 1] |= carry << (63 - (self.size - 1) % 64);
    }

    pub fn rotate_right(&mut self) {
        let l = self.data.len();
        let mut carry = 0;
        for i in 0..l {
            let new_carry = if i == l - 1 {
                (self.data[i] & (1 << (63 - (self.size - 1) % 64)) != 0) as u64
            } else {
                self.data[i] & 1
            };
            self.data[i] = (self.data[i] >> 1) | carry << 63;
            carry = new_carry;
        }
        self.data[l - 1] &= !(1 << (63 - self.size % 64));
        self.data[0] |= carry << 63;
    }
}

impl BitOrAssign<&Self> for BitArray {
    fn bitor_assign(&mut self, rhs: &Self) {
        for i in 0..self.data.len() {
            self.data[i] |= rhs.data[i];
        }
    }
}

#[cfg(test)]
mod test {
    use super::BitArray;

    #[test]
    fn create() {
        let bitarray = BitArray::new(63);
        assert_eq!(bitarray.data.len(), 1);
        assert_eq!(bitarray.size, 63);
        assert_eq!(bitarray.count_ones(), 0);

        let bitarray = BitArray::new(64);
        assert_eq!(bitarray.data.len(), 1);
        assert_eq!(bitarray.size, 64);
        assert_eq!(bitarray.count_ones(), 0);

        let bitarray = BitArray::new(65);
        assert_eq!(bitarray.data.len(), 2);
        assert_eq!(bitarray.size, 65);
        assert_eq!(bitarray.count_ones(), 0);

        let bitarray = BitArray::new(100);
        assert_eq!(bitarray.data.len(), 2);
        assert_eq!(bitarray.size, 100);
        assert_eq!(bitarray.count_ones(), 0);
    }

    #[test]
    fn set() {
        let mut bitarray = BitArray::new(100);

        assert!(!bitarray.get(0));
        bitarray.set(0);
        assert_eq!(bitarray.data[0], 1 << 63);
        assert_eq!(bitarray.data[1], 0);
        assert!(bitarray.get(0));
        assert_eq!(bitarray.count_ones(), 1);

        assert!(!bitarray.get(63));
        bitarray.set(63);
        assert_eq!(bitarray.data[0], 1 << 63 | 1);
        assert_eq!(bitarray.data[1], 0);
        assert!(bitarray.get(63));
        assert_eq!(bitarray.count_ones(), 2);

        assert!(!bitarray.get(64));
        bitarray.set(64);
        assert_eq!(bitarray.data[0], 1 << 63 | 1);
        assert_eq!(bitarray.data[1], 1 << 63);
        assert!(bitarray.get(64));
        assert_eq!(bitarray.count_ones(), 3);

        assert!(!bitarray.get(70));
        bitarray.set(70);
        assert_eq!(bitarray.data[0], 1 << 63 | 1);
        assert_eq!(bitarray.data[1], 1 << 63 | 1 << 57);
        assert!(bitarray.get(70));
        assert_eq!(bitarray.count_ones(), 4);
    }

    #[test]
    fn count() {
        let mut bitarray = BitArray::new(100);

        assert_eq!(bitarray.count_ones(), 0);

        bitarray.set(0);
        assert_eq!(bitarray.count_ones(), 1);

        bitarray.set(63);
        assert_eq!(bitarray.count_ones(), 2);

        bitarray.set(64);
        assert_eq!(bitarray.count_ones(), 3);

        bitarray.set(70);
        assert_eq!(bitarray.count_ones(), 4);
    }

    #[test]
    fn clear() {
        let mut bitarray = BitArray::new(100);

        bitarray.set(0);
        bitarray.set(63);
        bitarray.set(64);
        bitarray.set(70);

        bitarray.clear(63);
        assert_eq!(bitarray.count_ones(), 3);
        assert!(!bitarray.get(63));

        bitarray.clear(64);
        assert_eq!(bitarray.count_ones(), 2);
        assert!(!bitarray.get(64));

        bitarray.clear(70);
        assert_eq!(bitarray.count_ones(), 1);
        assert!(!bitarray.get(70));

        bitarray.clear(0);
        assert_eq!(bitarray.count_ones(), 0);
        assert!(!bitarray.get(0));
    }

    #[test]
    fn clear_all() {
        let mut bitarray1 = BitArray::new(100);
        let mut bitarray2 = BitArray::new(100);

        bitarray1.set(0);
        bitarray1.set(63);
        bitarray1.set(64);
        bitarray1.set(70);

        bitarray2.set(0);
        bitarray2.set(20);
        bitarray2.set(70);
        bitarray2.set(80);

        bitarray1.clear_all(&bitarray2);

        assert_eq!(bitarray1.count_ones(), 2);
        assert!(bitarray1.get(63));
        assert!(bitarray1.get(64));
    }

    #[test]
    fn bitor_assign() {
        let mut bitarray1 = BitArray::new(100);
        let mut bitarray2 = BitArray::new(100);

        bitarray1.set(0);
        bitarray1.set(63);
        bitarray1.set(64);
        bitarray1.set(70);

        bitarray2.set(0);
        bitarray2.set(20);
        bitarray2.set(70);
        bitarray2.set(80);

        bitarray1 |= &bitarray2;

        assert_eq!(bitarray1.count_ones(), 6);
        assert!(bitarray1.get(0));
        assert!(bitarray1.get(20));
        assert!(bitarray1.get(63));
        assert!(bitarray1.get(64));
        assert!(bitarray1.get(70));
        assert!(bitarray1.get(80));
    }

    #[test]
    fn rotate_left() {
        let mut bitarray = BitArray::new(100);

        bitarray.set(0);
        bitarray.set(63);
        bitarray.set(64);
        bitarray.set(70);

        bitarray.rotate_left();

        assert_eq!(bitarray.count_ones(), 4);
        assert!(bitarray.get(62));
        assert!(bitarray.get(63));
        assert!(bitarray.get(69));
        assert!(bitarray.get(99));
    }

    #[test]
    fn rotate_right() {
        let mut bitarray = BitArray::new(100);

        bitarray.set(0);
        bitarray.set(63);
        bitarray.set(64);
        bitarray.set(70);

        bitarray.rotate_right();

        assert_eq!(bitarray.count_ones(), 4);
        assert!(bitarray.get(1));
        assert!(bitarray.get(64));
        assert!(bitarray.get(65));
        assert!(bitarray.get(71));

        bitarray.set(98);
        bitarray.set(99);

        bitarray.rotate_right();

        assert_eq!(bitarray.count_ones(), 6);
        assert!(bitarray.get(0));
        assert!(bitarray.get(2));
        assert!(bitarray.get(65));
        assert!(bitarray.get(66));
        assert!(bitarray.get(72));
        assert!(bitarray.get(99));

        bitarray.rotate_right();

        assert_eq!(bitarray.count_ones(), 6);
        assert!(bitarray.get(0));
        assert!(bitarray.get(1));
        assert!(bitarray.get(3));
        assert!(bitarray.get(66));
        assert!(bitarray.get(67));
        assert!(bitarray.get(73));
    }
}
