# Freivald-Rust

A rust implementation of Freivald's algorithm on matrix multiplication

### Get Started
```rust
cargo test -- --nocapture
cargo bench -- --nocapture
```

/// TODO
function freivald_verify (matrix: a, matrix b, matrix result) -> bool
	getR
	getVector
	compute left
	compute right
	equate

// O(n) generates vector x, for random r in Fp
function getVector(function(self*self) , size n, int r ) -> vec:<>...

function getRandomRInP() -> int

Benchmark

Helpful links: 
- https://www.reddit.com/r/rust/comments/fmigwt/benchmarking_various_crates_for_matrix/