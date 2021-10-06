[![crates.io](https://img.shields.io/crates/v/round_robin_tournament.svg)](https://crates.io/crates/round_robin_tournament)
[![Documentation](https://docs.rs/round_robin_tournament/badge.svg)](https://docs.rs/round_robin_tournament)
[![Workflow](https://github.com/flocksserver/round_robin_tournament/workflows/Rust/badge.svg)](https://github.com/flocksserver/round_robin_tournament/workflows/Rust/badge.svg)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)


# Round Robin Tournament

Small library to get a round robin schedule for n players.

## Example

```rust
use round_robin_tournament::round_robin_tournament::draw;

let tournament: Vec<Vec<(u32, u32)>> = draw(10);
// First round with 5 matches
let first_round = tournament.first().unwrap();
// First match with player id 0 against player id 9
let first_match = first_round.first().unwrap();
//Full tournament
/*
[(0, 9), (1, 8), (2, 7), (3, 6), (4, 5)]
[(1, 9), (2, 0), (3, 8), (4, 7), (5, 6)]
[(2, 9), (3, 1), (4, 0), (5, 8), (6, 7)]
[(3, 9), (4, 2), (5, 1), (6, 0), (7, 8)]
[(4, 9), (5, 3), (6, 2), (7, 1), (8, 0)]
[(5, 9), (6, 4), (7, 3), (8, 2), (0, 1)]
[(6, 9), (7, 5), (8, 4), (0, 3), (1, 2)]
[(7, 9), (8, 6), (0, 5), (1, 4), (2, 3)]
[(8, 9), (0, 7), (1, 6), (2, 5), (3, 4)]
*/

```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE_APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE_MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.