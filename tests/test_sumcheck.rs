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

type MultiPoly = SparsePolynomial<ScalarField, SparseTerm>;

lazy_static! {
	// g = 2(x_1)^3 + (x_1)(x_3) + (x_2)(x_3)
	static ref G_0: MultiPoly = SparsePolynomial::from_coefficients_vec(
		3,
		vec![
			(2u32.into(), SparseTerm::new(vec![(0, 3)])),
			(1u32.into(), SparseTerm::new(vec![(0, 1), (2, 1)])),
			(1u32.into(), SparseTerm::new(vec![(1, 1), (2, 1)])),
		],
	);
}

#[rstest]
#[case(&G_0)]
fn verify_test(#[case] p: &MultiPoly) {
	let result: BigInteger256 = p
		.evaluate(&vec![2u32.into(), 3u32.into(), 4u32.into()])
		.into_repr();

	assert_eq!(result, 36.into());
}
