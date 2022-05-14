#[macro_use]
extern crate lazy_static;

// scalar field
use ark_bls12_381::Fr;
use ndarray::{arr2, Array2};
use rstest::rstest;
use thaler::freivald;

lazy_static! {
	static ref MATRIX_A: Array2<Fr> = arr2(&[[1.into(), 2.into()], [3.into(), 4.into()]]);
	static ref MATRIX_A_DOT_A: Array2<Fr> = arr2(&[[7.into(), 10.into()], [15.into(), 22.into()]]);
	static ref MATRIX_B: Array2<Fr> = arr2(&[[1.into(), 2.into(), 3.into()], [4.into(), 5.into(), 6.into()], [7.into(), 8.into(), 9.into()]]);
	static ref MATRIX_B_DOT_B: Array2<Fr> = arr2(&[[30.into(), 36.into(), 42.into()], [66.into(), 81.into(), 96.into()], [102.into(), 126.into(), 150.into()]]);
	// Large matrices
	static ref MATRIX_C: Array2<Fr> = Array2::<Fr>::ones((200, 200));
	static ref MATRIX_C_DOT_C: Array2<Fr> = MATRIX_C.dot(&Array2::<Fr>::ones((200, 200)));
}

#[rstest]
#[case(&MATRIX_A, &MATRIX_A, &MATRIX_A_DOT_A)]
#[case(&MATRIX_B, &MATRIX_B, &MATRIX_B_DOT_B)]
#[case(&MATRIX_C, &MATRIX_C, &MATRIX_C_DOT_C)]
fn dumb_verify_test(#[case] a: &Array2<Fr>, #[case] b: &Array2<Fr>, #[case] c: &Array2<Fr>) {
	assert!(freivald::dumb_verify(a, b, c));
}

#[rstest]
#[case(&MATRIX_A, &MATRIX_A, &MATRIX_A_DOT_A)]
#[case(&MATRIX_B, &MATRIX_B, &MATRIX_B_DOT_B)]
#[case(&MATRIX_C, &MATRIX_C, &MATRIX_C_DOT_C)]
fn freivald_verify_test(#[case] a: &Array2<Fr>, #[case] b: &Array2<Fr>, #[case] c: &Array2<Fr>) {
	assert!(freivald::freivald_verify(a, b, c));
}
