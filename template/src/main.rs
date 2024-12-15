#![allow(unused)]
use grid::*;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fs;
use util::*;

mod grid;
mod util;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();
    // let grid = grid::read_to_grid("input.txt").expect("Could not read file");
}
