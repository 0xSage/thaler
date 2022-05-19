use ark_bls12_381::Fr as ScalarField;
use ark_ff::{BigInteger, Field, FpParameters, PrimeField};
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
			g: g.clone(),  // unmodified, can be just a static ref
			r_vec: vec![], // modified each time
		}
	}

	// Given polynomial g
	// Fix x_0
	// Evaluate over remaining x...
	// Returns: a univariate polynomial
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

	// Public helper fns for prover
	// Evaluates a term with a fixed var, returning (new coefficent, fixed term)
	// point is [r1, r2, (rj, x, x, x,)]
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

	// evaluates g_j over a vector of points
	// point is the later half of all points
	// construct the univariate polynomial term by term
	// returns univariate::Polynomial with x_0 fixed
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
	pub fn sum_g(&self) -> ScalarField {
		let v = self.g.num_vars();
		let n = 2u32.pow(v as u32);
		(0..n)
			.map(|n| self.g.evaluate(&n_to_vec(n as usize, v)))
			.sum()
	}
}

// Verifier procedures
// Verifier: Random r over large field F
pub fn get_r() -> Option<ScalarField> {
	let mut rng = rand::thread_rng();
	let r: ScalarField = rng.gen();
	Some(r)
}

// SumCheck Protocol

// Assuming g and c_1 are from prover
pub fn verify(g: &MultiPoly, c_1: ScalarField) -> bool {
	// 1st round
	let mut p = Prover::new(g);
	let mut gi = p.gen_uni_polynomial(None);
	let mut expected_c = gi.evaluate(&0u32.into()) + gi.evaluate(&1u32.into());
	assert_eq!(c_1, expected_c);
	println!("expected c: {:?} ", expected_c.into_repr());
	// todo, build helper function to let check degree
	// assert!(g1.degree <=  );
	// middle steps
	for _ in 1..p.g.num_vars() {
		// prev S
		let r = get_r();
		expected_c = gi.evaluate(&r.unwrap());

		println!("prev g evaluated on r: {:?}", expected_c.into_repr());
		gi = p.gen_uni_polynomial(r);
		let new_c = gi.evaluate(&0u32.into()) + gi.evaluate(&1u32.into());
		println!("new c check: {:?}", new_c.into_repr());
		assert_eq!(expected_c, new_c);
	}
	// final check
	let r = get_r();
	expected_c = gi.evaluate(&r.unwrap());
	// println!("prev g evaluated on r: {:?}", expected_c.into_repr());
	// println!("new c check: {:?}", new_c.into_repr());
	p.r_vec.push(r.unwrap());
	let new_c = p.g.evaluate(&p.r_vec);
	assert_eq!(expected_c, new_c);
	true
}

// TODO later, slow verify is just computing g...
pub fn slow_verify() -> bool {
	true
}
