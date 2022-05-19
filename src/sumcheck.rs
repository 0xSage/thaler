use ark_bls12_381::Fr as ScalarField;
use ark_ff::{BigInteger, Field, FpParameters, PrimeField};
use ark_poly::polynomial::multivariate::{SparsePolynomial, SparseTerm, Term};
use ark_poly::polynomial::univariate::SparsePolynomial as UniSparsePolynomial;
use ark_poly::polynomial::{MVPolynomial, Polynomial};
use ark_std::cfg_into_iter;

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
	// TODO: just use lifetime reference of g instead of cloning
	g: MultiPoly,
	// polynomial with j-1 variables fixed by r
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
				println!(
					"now evaluating: {:?}, {:?}",
					x[0].into_repr(),
					x[1].into_repr(),
					// x[2].into_repr()
				);
				println!("r vec looks like: {:?}", self.r_vec);
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
		println!("        at term: {:?}", term);
		let mut fixed_term: Option<SparseTerm> = None;
		let coeff: ScalarField =
			cfg_into_iter!(term).fold(1u32.into(), |product, (var, power)| match *var {
				// if variable needs to be fixed, not evaluated
				j if j == self.r_vec.len() => {
					fixed_term = Some(SparseTerm::new(vec![(j, *power)]));
					product
				}
				// if variable is already defined in r
				j if j < self.r_vec.len() => self.r_vec[j].pow(&[*power as u64]) * product,
				// if variable is being permutted on defined in r
				_ => point[*var - self.r_vec.len()].pow(&[*power as u64]) * product,
			});
		println!(
			"             evaluate term product: {:?}",
			coeff.into_repr()
		);
		println!("             simplified term: {:?}", fixed_term);
		(coeff, fixed_term)
	}

	// evaluates g_j over a vector of points
	// point is the later half of all points
	// returns univariate::Polynomial with x_0 fixed
	pub fn evaluate_gj(&self, points: Vec<ScalarField>) -> UniPoly {
		// term coefficient
		// let unipoly_coefficients: Vec<(usize, ScalarField)> = cfg_into_iter!(self.g.terms())
		// 	.map(|(coeff, term)| {
		// 		let (coeff_eval, fixed_term) = self.evaluate_term(&term, &points);
		// 		match fixed_term {
		// 			// (degree, coefficient)
		// 			None => (0, *coeff * coeff_eval),
		// 			_ => (fixed_term.unwrap().degree(), *coeff * coeff_eval),
		// 		}
		// 	})
		// 	.filter(|(_, coefficient)| (*coefficient != 0.into())) //filter out coefficients = 0,
		// 	.collect();

		// easier to fold it onto itself  Vec<(usize, ScalarField)>
		// construct the univariate polynomial term by term
		let unipoly_coefficients: UniPoly = cfg_into_iter!(self.g.terms()).fold(
			UniPoly::from_coefficients_vec(vec![]),
			|sum, (coeff, term)| {
				let (coeff_eval, fixed_term) = self.evaluate_term(&term, &points);
				let curr = match fixed_term {
					// (degree, coefficient), null fixed terms are degree 0 obviously
					None => UniPoly::from_coefficients_vec(vec![(0, *coeff * coeff_eval)]),
					_ => UniPoly::from_coefficients_vec(vec![(
						fixed_term.unwrap().degree(),
						*coeff * coeff_eval,
					)]),
				};
				curr + sum
			},
		);

		// for i in (0..unipoly_coefficients.len()) {
		// 	println!(
		// 		"    unipoly term {:?}: degree {:?}: coefficient {:?}",
		// 		i,
		// 		unipoly_coefficients[i].0,
		// 		unipoly_coefficients[i].1.into_repr()
		// 	);
		// }
		// UniPoly::from_coefficients_vec(unipoly_coefficients)
		unipoly_coefficients
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
pub fn get_r() -> i128 {
	// TODO implement this
	1.into()
}

// Verifier: Evaluates univariate polynomial g at x
pub fn eval_gx(x: i128, g: Vec<i128>) -> i128 {
	0
}

// SumCheck Protocol

// Verifies the H against provers claim in O(v + [cost to evaluate g at single input])
// c1: prover claim of the value H defined in eq 4.1
// g: a v variate polynomial defined over finite field F. in equation 4.1
// g: index is degree
pub fn verify(c_1: i128, g: MultiPoly) -> bool {
	true
}

// TODO later, slow verify is just computing g...
pub fn slow_verify() -> bool {
	true
}

// 1. partial sum for the 1st dimension, a univariate polynomial
// 2. Tells verifiier their claim
// 3. verifier has to heck is this the right practical sum for hte first variable?
// 4. prover fixes the first coordinates, sums everything but 2nd coordinate, p2
// 5. summing over free variable, to check
// 6. verifier keesp probing randomness,
// 7. last step is lack coordinate check
// 8. verifier checks last polynomial is the original that was evaluated.
