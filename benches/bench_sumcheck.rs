#![feature(test)]

#[macro_use]
extern crate lazy_static;

extern crate test;
use ark_bls12_381::Fr as ScalarField;
use ark_poly::polynomial::multivariate::{SparsePolynomial, SparseTerm, Term};
use ark_poly::polynomial::{MVPolynomial, Polynomial};
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
			)
		],
	);
	static ref G_1_SUM: ScalarField = sumcheck::Prover::new(&G_1).slow_sum_g();
}

// a gi lookup table
fn build_gi_lookup() -> Vec<sumcheck::UniPoly> {
	let r: Option<ScalarField> = Some(2u32.into());
	let mut lookup = vec![];
	let mut p = sumcheck::Prover::new(&G_1);
	let mut gi = p.gen_uni_polynomial(None);
	lookup.push(gi.clone());
	for _ in 1..p.g.num_vars() {
		gi = p.gen_uni_polynomial(r);
		lookup.push(gi.clone());
	}
	lookup
}

// Steps being benchmarked
fn verifier_steps_only(gi_lookup: &Vec<sumcheck::UniPoly>, r: Option<ScalarField>) {
	// initial round
	let p = sumcheck::Prover::new(&G_1);
	let mut gi = gi_lookup[0].clone();
	let mut expected_c = gi.evaluate(&0u32.into()) + gi.evaluate(&1u32.into());
	assert_eq!(*G_1_SUM, expected_c);
	let lookup_degree = sumcheck::max_degrees(&G_1);
	assert!(gi.degree() <= lookup_degree[0]);
	// middle rounds
	for j in 1..p.g.num_vars() {
		expected_c = gi.evaluate(&r.unwrap());
		gi = gi_lookup[j].clone();
		let new_c = gi.evaluate(&0u32.into()) + gi.evaluate(&1u32.into());
		assert_eq!(expected_c, new_c);
		assert!(gi.degree() <= lookup_degree[j]);
	}
	// final round
	expected_c = gi.evaluate(&r.unwrap());
	let new_c = G_1.evaluate(&vec![r.unwrap(); p.g.num_vars()]);
	assert_eq!(expected_c, new_c);
}

// Verifier benchmark
#[bench]
fn sumcheck_test(b: &mut Bencher) {
	let gi_lookup = build_gi_lookup();
	let r: Option<ScalarField> = Some(2u32.into());

	b.iter(|| verifier_steps_only(&gi_lookup, r));
}

#[bench]
fn slow_sumcheck_test(b: &mut Bencher) {
	let p = sumcheck::Prover::new(&G_1);
	b.iter(|| p.slow_sum_g());
}
