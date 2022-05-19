#[macro_use]
extern crate lazy_static;

// scalar field
use ark_bls12_381::Fr as ScalarField;
use ark_ff::biginteger::BigInteger256;
use ark_ff::{BigInteger, Field, FpParameters, PrimeField};
use ark_poly::polynomial::multivariate::{SparsePolynomial, SparseTerm, Term};
use ark_poly::polynomial::{MVPolynomial, Polynomial};
use ndarray::{arr2, Array2};
use rstest::rstest;
use thaler::sumcheck;

lazy_static! {
	static ref MATRIX_A: Array2<ScalarField> = arr2(&[[1.into(), 2.into()], [3.into(), 4.into()]]);
}

// #[rstest]
// #[case(&MATRIX_A, &MATRIX_A, &MATRIX_A_DOT_A)]
#[test]
fn verify_test() {
	let p: SparsePolynomial<ScalarField, SparseTerm> = SparsePolynomial::from_coefficients_vec(
		3,
		vec![
			("2".parse().unwrap(), Term::new(vec![(0, 3)])),
			("1".parse().unwrap(), Term::new(vec![(0, 1), (2, 1)])),
			("1".parse().unwrap(), Term::new(vec![(1, 1), (2, 1)])),
		],
	);

	let result: BigInteger256 = p
		.evaluate(&vec![2u32.into(), 3u32.into(), 4u32.into()])
		.into_repr();

	assert_eq!(result, 36.into());
}
