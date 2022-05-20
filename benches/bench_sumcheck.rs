#![feature(test)]

#[macro_use]
extern crate lazy_static;

extern crate test;
use ark_bls12_381::Fr as ScalarField;
use ark_poly::polynomial::multivariate::{SparsePolynomial, SparseTerm, Term};
use ark_poly::polynomial::MVPolynomial;
use test::Bencher;
use thaler::sumcheck;

lazy_static! {
	// Benchmarking with a large polynomial
	static ref G_1: sumcheck::MultiPoly = SparsePolynomial::from_coefficients_vec(
		7,
		vec![
			(2u32.into(), SparseTerm::new(vec![(0, 3)])),
			(1u32.into(), SparseTerm::new(vec![(0, 1), (2, 1), (3, 4)])),
			(1u32.into(), SparseTerm::new(vec![(1, 1), (2, 1), (3, 4)])),
			(1u32.into(), SparseTerm::new(vec![(3, 1), (2, 1), (4, 3)])),
			(
				1u32.into(),
				SparseTerm::new(vec![(4, 3), (1, 1), (2, 3), (3, 3)])
			),
			(
				1u32.into(),
				SparseTerm::new(vec![(5, 3), (1, 1), (2, 3), (3, 3)])
			),
			(
				1u32.into(),
				SparseTerm::new(vec![(6, 5), (1, 1), (2, 3), (3, 3), (4, 4), (5, 5)])
			),
			(
				1u32.into(),
				SparseTerm::new(vec![(6, 6), (1, 1), (2, 3), (3, 3), (4, 4), (5, 5)])
			),
			(
				1u32.into(),
				SparseTerm::new(vec![(6, 7), (1, 1), (2, 3), (3, 3), (4, 4), (5, 5)])
			),
			(
				1u32.into(),
				SparseTerm::new(vec![(6, 8), (1, 1), (2, 3), (3, 3), (4, 4), (5, 5)])
			),
			(
				1u32.into(),
				SparseTerm::new(vec![(6, 9), (1, 3), (2, 5), (3, 3), (4, 4), (5, 5)])
			),
			(
				1u32.into(),
				SparseTerm::new(vec![(6, 10), (1, 4), (2, 3), (3, 6), (4, 4), (5, 5)])
			),
		],
	);
	static ref G_1_SUM: ScalarField = sumcheck::Prover::new(&G_1).slow_sum_g();
}

#[bench]
fn sumcheck_test(b: &mut Bencher) {
	b.iter(|| sumcheck::verify(&G_1, *G_1_SUM));
}

#[bench]
fn slow_sumcheck_test(b: &mut Bencher) {
	b.iter(|| sumcheck::slow_verify(&G_1, *G_1_SUM));
}
