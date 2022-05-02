#[macro_use]
extern crate lazy_static;

use freivald;
use ndarray::{arr2, Array2};
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;
use rstest::rstest;

lazy_static! {
	static ref MATRIX_A: Array2<i64> = arr2(&[[1, 2], [3, 4]]);
	static ref MATRIX_A_DOT_A: Array2<i64> = arr2(&[[7, 10], [15, 22]]);
	static ref MATRIX_B: Array2<i64> = arr2(&[[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
	static ref MATRIX_B_DOT_B: Array2<i64> = arr2(&[[30, 36, 42], [66, 81, 96], [102, 126, 150]]);
	// Very large matrices
	static ref MATRIX_C: Array2<i64> = Array2::random((300, 300), Uniform::new(0, 10));
	static ref MATRIX_D: Array2<i64> = Array2::<i64>::ones((300, 300));
	static ref MATRIX_C_DOT_D: Array2<i64> = MATRIX_C.dot(&Array2::<i64>::ones((300, 300)));
}

#[rstest]
#[case(&MATRIX_A, &MATRIX_A, &MATRIX_A_DOT_A)]
#[case(&MATRIX_B, &MATRIX_B, &MATRIX_B_DOT_B)]
#[case(&MATRIX_C, &MATRIX_D, &MATRIX_C_DOT_D)]
fn dumb_verify_test(#[case] a: &Array2<i64>, #[case] b: &Array2<i64>, #[case] c: &Array2<i64>) {
	assert!(freivald::dumb_verify(a, b, c));
}

#[rstest]
#[case(&MATRIX_A, &MATRIX_A, &MATRIX_A_DOT_A)]
fn freivald_verify_test(#[case] a: &Array2<i64>, #[case] b: &Array2<i64>, #[case] c: &Array2<i64>) {
	// TODO
	assert!(true);
}
