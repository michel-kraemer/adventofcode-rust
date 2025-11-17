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

🏃‍♂️‍➡️ Try to optimize performance (to the best of my knowledge)

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

## License

The solutions are released under the **MIT license**. See the
[LICENSE](LICENSE) file for more information.
