#[macro_use]
extern crate lazy_static;

use rstest::rstest;
use std::collections::HashMap;
use thaler::lagrange;

lazy_static! {
	static ref F_2: Vec<i128> = Vec::from([1, 2, 1, 4]);
	static ref R_0: Vec<i128> = Vec::from([0, 0]);
	static ref R_1: Vec<i128> = Vec::from([0, 2]);
	static ref R_2: Vec<i128> = Vec::from([3, 4]);
	static ref R_3: Vec<i128> = Vec::from([4, 1]);
	// Test with v=3 as well...
}

#[rstest]
#[case(&F_2, &R_0, 1, 5)]
#[case(&F_2, &R_1, 3, 5)]
#[case(&F_2, &R_2, 4, 5)]
#[case(&F_2, &R_3, 0, 5)]
fn slow_lagrange_test(
	#[case] fw: &Vec<i128>,
	#[case] r: &Vec<i128>,
	#[case] expected: i128,
	#[case] p: i128,
) {
	assert_eq!(lagrange::slow_mle(fw, r, p), expected);
}

#[rstest]
#[case(&F_2, &R_0, 1, 5)]
#[case(&F_2, &R_1, 3, 5)]
#[case(&F_2, &R_2, 4, 5)]
#[case(&F_2, &R_3, 0, 5)]
fn stream_lagrange_test(
	#[case] fw: &Vec<i128>,
	#[case] r: &Vec<i128>,
	#[case] expected: i128,
	#[case] p: i128,
) {
	assert_eq!(lagrange::stream_mle(fw, r, p), expected);
}

#[rstest]
#[case(&F_2, &R_0, 1, 5)]
#[case(&F_2, &R_1, 3, 5)]
#[case(&F_2, &R_2, 4, 5)]
#[case(&F_2, &R_3, 0, 5)]
fn memoize_test(
	#[case] fw: &Vec<i128>,
	#[case] r: &Vec<i128>,
	#[case] expected: i128,
	#[case] p: i128,
) {
	assert_eq!(lagrange::dynamic_mle(fw, r, p), expected);
}
