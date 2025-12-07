use std::fs;

fn dfs(
    x: usize,
    y: usize,
    grid: &[u8],
    width: usize,
    height: usize,
    cache: &mut Vec<usize>,
) -> usize {
    if y == height {
        return 1;
    }
    let mut result = cache[y * width + x];
    if result > 0 {
        return result;
    }
    if grid[y * width + x] == b'^' {
        // go left and right
        result += dfs(x - 1, y, grid, width, height, cache);
        result += dfs(x + 1, y, grid, width, height, cache);

        // Only save entries in the cache if we've hit a splitter. This way,
        // we can later calculate how many splitters are reachable (part 1).
        // As a side effect, we need to access the array less often, which
        // saves a bit of time.
        cache[y * width + x] = result;
    } else {
        // go downwards
        result += dfs(x, y + 1, grid, width, height, cache);
    }
    result
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();
    let width = lines[0].len();
    let height = lines.len();
    let grid = lines
        .iter()
        .flat_map(|l| l.as_bytes())
        .copied()
        .collect::<Vec<_>>();

    let start = (width / 2, 0);

    // perform DFS to calculate the number of possible ways (i.e. timelines)
    // a single particle can take
    let mut cache = vec![0; grid.len()];
    let total2 = dfs(start.0, start.1, &grid, width, height, &mut cache);

    // part 1 - the number of cache entries is the total number of reachable
    // splitters
    let total1 = cache.iter().filter(|c| **c > 0).count();
    println!("{total1}");

    println!("{total2}");
}
