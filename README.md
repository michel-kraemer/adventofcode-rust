<div align="center">
	<img width="100%" src="./.github/banner.png" alt="Advent of Code">
	<br>
  <br>
</div>

> These are my solutions to the annual [Advent of Code](https://adventofcode.com)
challenge.

## Goals

🦀 Implement everything in Rust

👍 Find generalized solutions that work with every input

⛓️‍💥 Avoid 3rd party dependencies (if possible and reasonable)

🏃‍♂️‍➡️ Try to optimize performance (to the best of my knowledge and if it doesn't
   conflict with the other goals). See [benchmarks](#benchmarks) below.

🤓 Find clever solutions to complex problems that others might not have found.
  Some highlights I'm particularly proud of:
* [Day 25 of 2018](2018/day25/src/main.rs): Combining DBSCAN with a spatial
  index based on Morton indices. Very fast!
* [Day 21 of 2023](2023/day21/src/main.rs): Applying
  [bit operations](2023/day21/src/bitarray.rs) to simulate the cellular
  automaton, which allowed me to implement a fast and generalized solution
  that should work on any input.
* [Day 25 of 2023](2023/day25/src/main.rs): Using
  [Brandes' algorithm](https://doi.org/10.1080/0022250X.2001.9990249) to
  quickly calculate the betweenness centrality of graph nodes.
* [Day 10 of 2025](2025/day10/src/main.rs): While most people approached this
  problem using a solver (such as [S3](https://github.com/Z3Prover/z3) or
  [good_lp](https://github.com/rust-or/good_lp)), I found a solution that
  works completely without a 3rd party dependency. I implemented a DFS in
  which I use heuristics to prune as many branches as possible. This is
  definitely not the fastest approach possible, but it gets the job done in
  reasonable time and is certainly quite unique. See my
  [Reddit comment](https://www.reddit.com/r/adventofcode/comments/1pity70/comment/ntb36sb/)
  for more information.

## Terminal visualizations

I've created terminal visualizations of some puzzles for fun:

<table>
  <tr>
    <td align="center">
      <img src="2019/day13/aoc_2019_day13.gif" height="200px" /><br />
      <a href="2019/day13/src/main.rs">2019 - Day 13: Care Package</a>
    </td>
    <td align="center">
      <img src="2024/day15/aoc_2024_day15.gif" height="200px" /><br />
      <a href="2024/day15/src/main.rs">2024 - Day 15: Warehouse Woes</a>
    </td>
  </tr>
  <tr>
    <td align="center">
      <img src="2025/day04/aoc_2025_day04.gif" height="200px" /><br />
      <a href="2025/day04/src/main.rs">2025 - Day 4: Printing Department</a>
    </td>
    <td align="center">
      <img src="2025/day12/aoc_2025_day12.gif" height="200px" /><br />
      <a href="2025/day12/src/main.rs">2025 - Day 12: Christmas Tree Farm</a>
    </td>
  </tr>
</table>

If you want to visualize your own input with my code, just change into the
solution's directory (e.g. `cd 2025/day12`) and run the following command:

```bash
AOC_VISUALIZE=true cargo run --release
```

## Benchmarks

All measurements were performed with my [cargo bench-aoc tool](./cargo-bench-aoc/) on a MacBook Pro 16″ 2023 with an Apple M3 Pro processor. I/O (reading input files and writing answers to the terminal) is **NOT** included in the benchmark results.

### 2015

<table>
<tr>
<td><strong><a href="./2015/day01/">Day 1:</a></strong></td><td>6µs</td>
<td><strong><a href="./2015/day02/">Day 2:</a></strong></td><td>42µs</td>
<td><strong><a href="./2015/day03/">Day 3:</a></strong></td><td>415µs</td>
<td><strong><a href="./2015/day04/">Day 4:</a></strong></td><td>633ms</td>
<td><strong><a href="./2015/day05/">Day 5:</a></strong></td><td>168µs</td>
</tr>
<tr>
<td><strong><a href="./2015/day06/">Day 6:</a></strong></td><td>6ms</td>
<td><strong><a href="./2015/day07/">Day 7:</a></strong></td><td>23µs</td>
<td><strong><a href="./2015/day08/">Day 8:</a></strong></td><td>7µs</td>
<td><strong><a href="./2015/day09/">Day 9:</a></strong></td><td>19µs</td>
<td><strong><a href="./2015/day10/">Day 10:</a></strong></td><td>16ms</td>
</tr>
<tr>
<td><strong><a href="./2015/day11/">Day 11:</a></strong></td><td>8ms</td>
<td><strong><a href="./2015/day12/">Day 12:</a></strong></td><td>125µs</td>
<td><strong><a href="./2015/day13/">Day 13:</a></strong></td><td>42µs</td>
<td><strong><a href="./2015/day14/">Day 14:</a></strong></td><td>23µs</td>
<td><strong><a href="./2015/day15/">Day 15:</a></strong></td><td>...</td>
</tr>
<tr>
<td><strong><a href="./2015/day16/">Day 16:</a></strong></td><td>...</td>
<td><strong><a href="./2015/day17/">Day 17:</a></strong></td><td>...</td>
<td><strong><a href="./2015/day18/">Day 18:</a></strong></td><td>...</td>
<td><strong><a href="./2015/day19/">Day 19:</a></strong></td><td>...</td>
<td><strong><a href="./2015/day20/">Day 20:</a></strong></td><td>...</td>
</tr>
<tr>
<td><strong><a href="./2015/day21/">Day 21:</a></strong></td><td>...</td>
<td><strong><a href="./2015/day22/">Day 22:</a></strong></td><td>...</td>
<td><strong><a href="./2015/day23/">Day 23:</a></strong></td><td>...</td>
<td><strong><a href="./2015/day24/">Day 24:</a></strong></td><td>...</td>
<td><strong><a href="./2015/day25/">Day 25:</a></strong></td><td>...</td>
</tr>
</table>

## License

The solutions are released under the **MIT license**. See the
[LICENSE](LICENSE) file for more information.
