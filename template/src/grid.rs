#![allow(unused)]
use std::{
    char,
    fmt::{Display, Formatter},
    fs,
    ops::{Index, Range},
};

// Right, Down, Left, Up
pub const DIRS: [(i64, i64); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

pub const CLOCKWISE: [(i64, i64); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

pub fn read_to_grid(filename: &str) -> Result<Grid<char>, std::io::Error> {
    Ok(fs::read_to_string(filename)?.to_grid())
}

pub trait ToGrid {
    fn to_grid(&self) -> Grid<char>;
}

impl ToGrid for &str {
    fn to_grid(&self) -> Grid<char> {
        Grid {
            grid: self.lines().map(|l| l.chars().collect()).collect(),
        }
    }
}

impl ToGrid for String {
    fn to_grid(&self) -> Grid<char> {
        self.as_str().to_grid()
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Grid<T: Copy> {
    pub grid: Vec<Vec<T>>,
}

impl<T: Copy> Grid<T> {
    #[inline]
    pub fn has(&self, x: i64, y: i64) -> bool {
        x >= 0 && x < self.width() && y >= 0 && y < self.height()
    }

    #[inline]
    pub fn width(&self) -> i64 {
        self.grid[0].len() as i64
    }

    #[inline]
    pub fn height(&self) -> i64 {
        self.grid.len() as i64
    }

    #[inline]
    pub fn len(&self) -> i64 {
        self.width() * self.height()
    }

    #[inline]
    pub fn get(&self, x: i64, y: i64) -> T {
        self.grid[y as usize][x as usize]
    }

    #[inline]
    pub fn set(&mut self, x: i64, y: i64, c: T) {
        self.grid[y as usize][x as usize] = c;
    }

    pub fn iter(&'_ self) -> GridIterator<'_, T> {
        GridIterator {
            grid: self,
            x: 0,
            y: 0,
        }
    }
}

pub struct GridIterator<'a, T: Copy> {
    grid: &'a Grid<T>,
    x: i64,
    y: i64,
}

impl<'a, T: Copy> Iterator for GridIterator<'a, T> {
    type Item = (i64, i64, T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.grid.height() {
            return None;
        }

        let c = self.grid.get(self.x, self.y);
        let r = (self.x, self.y, c);
        self.x += 1;
        if self.x >= self.grid.width() {
            self.x = 0;
            self.y += 1;
        }
        Some(r)
    }
}

impl<T: Copy + Display> Display for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
        for row in self.grid.iter() {
            for c in row.iter() {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
