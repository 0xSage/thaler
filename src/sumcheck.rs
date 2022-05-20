use ark_bls12_381::Fr as ScalarField;
use ark_ff::Field;
use ark_poly::polynomial::multivariate::{SparsePolynomial, SparseTerm, Term};
use ark_poly::polynomial::univariate::SparsePolynomial as UniSparsePolynomial;
use ark_poly::polynomial::{MVPolynomial, Polynomial};
use ark_std::cfg_into_iter;
use rand::Rng;

pub type MultiPoly = SparsePolynomial<ScalarField, SparseTerm>;
pub type UniPoly = UniSparsePolynomial<ScalarField>;

// Converts i into an index in {0,1}^v
pub fn n_to_vec(i: usize, n: usize) -> Vec<ScalarField> {
	format!("{:0>width$}", format!("{:b}", i), width = n)
		.chars()
		.map(|x| if x == '1' { 1.into() } else { 0.into() })
		.collect()
}

// Simulates memory of a single prover instance
#[derive(Debug, Clone)]
pub struct Prover {
	g: MultiPoly,
	r_vec: Vec<ScalarField>,
}

impl Prover {
	pub fn new(g: &MultiPoly) -> Self {
		Prover {
			g: g.clone(),
			r_vec: vec![],
		}
	}

	// Given polynomial g, fix Xj, evaluate over xj+1
	pub fn gen_uni_polynomial(&mut self, r: Option<ScalarField>) -> UniPoly {
		if r.is_some() {
			self.r_vec.push(r.unwrap());
		}
		let v = self.g.num_vars() - self.r_vec.len();
		(0..(2u32.pow(v as u32 - 1))).fold(
			UniPoly::from_coefficients_vec(vec![(0, 0u32.into())]),
			|sum, n| {
				let x = n_to_vec(n as usize, v);
				sum + self.evaluate_gj(n_to_vec(n as usize, v))
			},
		)
	}

	// Evaluates a term with a fixed univar, returning (new coefficent, fixed term)
	pub fn evaluate_term(
		&self,
		term: &SparseTerm,
		point: &Vec<ScalarField>,
	) -> (ScalarField, Option<SparseTerm>) {
		let mut fixed_term: Option<SparseTerm> = None;
		let coeff: ScalarField =
			cfg_into_iter!(term).fold(1u32.into(), |product, (var, power)| match *var {
				j if j == self.r_vec.len() => {
					fixed_term = Some(SparseTerm::new(vec![(j, *power)]));
					product
				}
				j if j < self.r_vec.len() => self.r_vec[j].pow(&[*power as u64]) * product,
				_ => point[*var - self.r_vec.len()].pow(&[*power as u64]) * product,
			});
		(coeff, fixed_term)
	}

	// Evaluates g_j over a vector permutation of points, folding all evaluated terms together into one univariate polynomial
	pub fn evaluate_gj(&self, points: Vec<ScalarField>) -> UniPoly {
		cfg_into_iter!(self.g.terms()).fold(
			UniPoly::from_coefficients_vec(vec![]),
			|sum, (coeff, term)| {
				let (coeff_eval, fixed_term) = self.evaluate_term(&term, &points);
				let curr = match fixed_term {
					None => UniPoly::from_coefficients_vec(vec![(0, *coeff * coeff_eval)]),
					_ => UniPoly::from_coefficients_vec(vec![(
						fixed_term.unwrap().degree(),
						*coeff * coeff_eval,
					)]),
				};
				curr + sum
			},
		)
	}

	// Sum all evaluations of polynomial `g` over boolean hypercube
	pub fn slow_sum_g(&self) -> ScalarField {
		let v = self.g.num_vars();
		let n = 2u32.pow(v as u32);
		(0..n)
			.map(|n| self.g.evaluate(&n_to_vec(n as usize, v)))
			.sum()
	}
}

// Verifier procedures
pub fn get_r() -> Option<ScalarField> {
	let mut rng = rand::thread_rng();
	let r: ScalarField = rng.gen();
	Some(r)
}

// A degree look up table for all variables in g
pub fn max_degrees(g: &MultiPoly) -> Vec<usize> {
	let mut lookup: Vec<usize> = vec![0; g.num_vars()];
	cfg_into_iter!(g.terms()).for_each(|(_, term)| {
		cfg_into_iter!(term).for_each(|(var, power)| {
			if *power > lookup[*var] {
				lookup[*var] = *power
			}
		});
	});
	lookup
}

// Assuming g and c_1 are from prover
pub fn verify(g: &MultiPoly, c_1: ScalarField) -> bool {
	// 1st round
	let mut p = Prover::new(g);
	let mut gi = p.gen_uni_polynomial(None);
	let mut expected_c = gi.evaluate(&0u32.into()) + gi.evaluate(&1u32.into());
	assert_eq!(c_1, expected_c);
	let lookup_degree = max_degrees(&g);
	assert!(gi.degree() <= lookup_degree[0]);

	// middle rounds
	for j in 1..p.g.num_vars() {
		let r = get_r();
		expected_c = gi.evaluate(&r.unwrap());
		gi = p.gen_uni_polynomial(r);
		let new_c = gi.evaluate(&0u32.into()) + gi.evaluate(&1u32.into());
		assert_eq!(expected_c, new_c);
		assert!(gi.degree() <= lookup_degree[j]);
	}
	// final round
	let r = get_r();
	expected_c = gi.evaluate(&r.unwrap());
	p.r_vec.push(r.unwrap());
	let new_c = p.g.evaluate(&p.r_vec);
	assert_eq!(expected_c, new_c);
	true
}

pub fn slow_verify(g: &MultiPoly, c_1: ScalarField) -> bool {
	let p = Prover::new(g);
	let manual_sum = p.slow_sum_g();
	manual_sum == c_1
}
