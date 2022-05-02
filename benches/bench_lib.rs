#![feature(test)]

#[macro_use]
extern crate lazy_static;

extern crate test;
use test::Bencher;

use freivald;
use ndarray::Array2;
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;

lazy_static! {
	static ref MATRIX_A: Array2<u64> = Array2::random((300, 300), Uniform::new(0, 10));
	static ref MATRIX_B: Array2<u64> = Array2::<u64>::ones((300, 300));
	static ref MATRIX_A_DOT_B: Array2<u64> = MATRIX_A.dot(&Array2::<u64>::ones((300, 300)));
}

#[bench]
fn dumb_verify_test(b: &mut Bencher) {
	b.iter(|| freivald::dumb_verify(&MATRIX_A, &MATRIX_B, &MATRIX_A_DOT_B));
	// TODO check if optimizations are messing up benchmarks, maybe blackbox it
}
