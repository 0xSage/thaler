#![feature(test)]

#[macro_use]
extern crate lazy_static;

extern crate test;
use test::Bencher;

use freivald;
use ndarray::{arr2, Array2};

lazy_static! {
	static ref MATRIX_A: Array2<i64> = arr2(&[[1, 2], [3, 4]]);
	static ref MATRIX_A_DOT_A: Array2<i64> = arr2(&[[7, 10], [15, 22]]);
	static ref MATRIX_B: Array2<i64> = arr2(&[[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
	static ref MATRIX_B_DOT_B: Array2<i64> = arr2(&[[30, 36, 42], [66, 81, 96], [102, 126, 150]]);
	// TODO generate huge matrices for existing test
}

#[bench]
fn dumb_verify_test(b: &mut Bencher) {
	b.iter(|| freivald::dumb_verify(&MATRIX_A, &MATRIX_A, &MATRIX_A_DOT_A));
	// TODO check if optimizations are messing up benchmarks
}
