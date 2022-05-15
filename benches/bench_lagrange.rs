#![feature(test)]

#[macro_use]
extern crate lazy_static;

extern crate test;
use test::Bencher;

use thaler::lagrange;

lazy_static! {
	static ref F_2: Vec<i128> = Vec::from([1, 2, 1, 4]);
	static ref R_0: Vec<i128> = Vec::from([0, 0]);
	static ref R_1: Vec<i128> = Vec::from([0, 2]);
	static ref R_2: Vec<i128> = Vec::from([3, 4]);
	static ref R_3: Vec<i128> = Vec::from([4, 1]);
}

#[bench]
fn slow_mle_test(b: &mut Bencher) {
	b.iter(|| lagrange::slow_mle(&F_2, &R_0, 5));
}

#[bench]
fn stream_mle_test(b: &mut Bencher) {
	b.iter(|| lagrange::stream_mle(&F_2, &R_0, 5));
}

#[bench]
fn dynamic_mle_test(b: &mut Bencher) {
	b.iter(|| lagrange::dynamic_mle(&F_2, &R_0, 5));
}
