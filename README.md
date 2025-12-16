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
   conflict with the other goals)

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

## License

The solutions are released under the **MIT license**. See the
[LICENSE](LICENSE) file for more information.
