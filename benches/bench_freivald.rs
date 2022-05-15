#![feature(test)]

#[macro_use]
extern crate lazy_static;

extern crate test;
use test::Bencher;

use ark_bls12_381::Fr;
use ndarray::Array2;
use thaler::freivald;

lazy_static! {
	static ref MATRIX_A: Array2<Fr> = Array2::<Fr>::ones((50, 50));
	static ref MATRIX_A_DOT_A: Array2<Fr> = MATRIX_A.dot(&Array2::<Fr>::ones((50, 50)));
}

#[bench]
fn dumb_verify_test(b: &mut Bencher) {
	b.iter(|| freivald::dumb_verify(&MATRIX_A, &MATRIX_A, &MATRIX_A_DOT_A));
}

#[bench]
fn freivald_verify_test(b: &mut Bencher) {
	b.iter(|| freivald::freivald_verify(&MATRIX_A, &MATRIX_A, &MATRIX_A_DOT_A));
}
