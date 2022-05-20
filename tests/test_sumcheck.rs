#[macro_use]
extern crate lazy_static;

use ark_bls12_381::Fr as ScalarField;
use ark_poly::polynomial::multivariate::{SparsePolynomial, SparseTerm, Term};
use ark_poly::polynomial::MVPolynomial;
use rstest::rstest;
use thaler::sumcheck;

lazy_static! {
	// g = 2(x_1)^3 + (x_1)(x_3) + (x_2)(x_3)
	static ref G_0: sumcheck::MultiPoly = SparsePolynomial::from_coefficients_vec(
		3,
		vec![
			(2u32.into(), SparseTerm::new(vec![(0, 3)])),
			(1u32.into(), SparseTerm::new(vec![(0, 1), (2, 1)])),
			(1u32.into(), SparseTerm::new(vec![(1, 1), (2, 1)])),
		],
	);
	static ref G_0_SUM: ScalarField = sumcheck::Prover::new(&G_0).slow_sum_g();
	// Test with a larger g
	static ref G_1: sumcheck::MultiPoly = SparsePolynomial::from_coefficients_vec(
		4,
		vec![
			(2u32.into(), SparseTerm::new(vec![(0, 3)])),
			(1u32.into(), SparseTerm::new(vec![(0, 1), (2, 1)])),
			(1u32.into(), SparseTerm::new(vec![(1, 1), (2, 1)])),
			(1u32.into(), SparseTerm::new(vec![(3, 1), (2, 1)])),
		],
	);
	static ref G_1_SUM: ScalarField = sumcheck::Prover::new(&G_1).slow_sum_g();
}

#[rstest]
#[case(&G_0, &G_0_SUM)]
#[case(&G_1, &G_1_SUM)]
fn sumcheck_test(#[case] p: &sumcheck::MultiPoly, #[case] c: &ScalarField) {
	assert!(sumcheck::verify(&p, *c));
}
