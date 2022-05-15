#[macro_use]
extern crate lazy_static;

use rstest::rstest;
use thaler::lagrange;

lazy_static! {
	static ref F_2: Vec<i128> = Vec::from([1, 2, 1, 4]);
	static ref R_0: Vec<i128> = Vec::from([0, 0]);
	static ref R_1: Vec<i128> = Vec::from([0, 2]);
	static ref R_2: Vec<i128> = Vec::from([3, 4]);
	static ref R_3: Vec<i128> = Vec::from([4, 1]);
	// Larger v tests
	static ref R_4: Vec<i128> = Vec::from([0, 0, 0]);
	static ref R_4_CHI: Vec<i128> = Vec::from([1, 0, 0, 0, 0, 0, 0, 0]);
	static ref R_5: Vec<i128> = Vec::from([1, 2, 3]);
	static ref R_5_CHI: Vec<i128> = Vec::from([0, 0, 0, 0, 2, -3, -4, 6]);
	static ref R_6: Vec<i128> = Vec::from([5, 5, 5]);
	static ref R_6_CHI: Vec<i128> = Vec::from([-64, 80, 80, -100, 80, -100, -100, 125]);
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
fn dynamic_mle_test(
	#[case] fw: &Vec<i128>,
	#[case] r: &Vec<i128>,
	#[case] expected: i128,
	#[case] p: i128,
) {
	assert_eq!(lagrange::dynamic_mle(fw, r, p), expected);
}

#[rstest]
#[case(&R_4, &R_4_CHI)]
#[case(&R_5,&R_5_CHI)]
#[case(&R_6, &R_6_CHI)]
fn memoize_test(#[case] r: &Vec<i128>, #[case] expected: &Vec<i128>) {
	assert_eq!(lagrange::memoize(r, r.len()), *expected);
}
