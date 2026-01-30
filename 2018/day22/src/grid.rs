#[derive(Clone)]
pub struct Grid<T>
where
    T: Copy,
{
    inner: Vec<T>,
    default: T,
    width: usize,
    height: usize,
}

impl<T> Grid<T>
where
    T: Copy,
{
    pub fn new(width: usize, height: usize, default: T) -> Self {
        Grid {
            inner: vec![default; width * height],
            default,
            width,
            height,
        }
    }

    fn ensure_size(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            return;
        }

        let nw = if x >= self.width {
            self.width * 2
        } else {
            self.width
        };
        let nh = if y >= self.height {
            self.height * 2
        } else {
            self.height
        };

        let mut ni = vec![self.default; nw * nh];
        for y in 0..self.height {
            ni[y * nw..y * nw + self.width]
                .copy_from_slice(&self.inner[y * self.width..y * self.width + self.width]);
        }

        self.inner = ni;
        self.width = nw;
        self.height = nh;
    }

    pub fn get(&mut self, x: usize, y: usize) -> T {
        if x < self.width && y < self.height {
            self.inner[y * self.width + x]
        } else {
            self.default
        }
    }

    pub fn insert(&mut self, x: usize, y: usize, v: T) {
        self.ensure_size(x, y);
        self.inner[y * self.width + x] = v;
    }
}
