use ark_bls12_381::Fr as ScalarField;
use ark_ff::{BigInteger, Field, FpParameters, PrimeField};
use ark_poly::polynomial::multivariate::{SparsePolynomial, SparseTerm, Term};
use ark_poly::polynomial::univariate::SparsePolynomial as UniSparsePolynomial;
use ark_poly::polynomial::{MVPolynomial, Polynomial};
use ark_std::cfg_into_iter;

pub type MultiPoly = SparsePolynomial<ScalarField, SparseTerm>;
pub type UniPoly = UniSparsePolynomial<ScalarField>;

// Simulates memory of a single prover instance
#[derive(Debug, Clone)]
pub struct Prover {
	// TODO: just use lifetime reference of g instead of cloning
	o_g: MultiPoly,
	// polynomial with j-1 variables fixed by r
	g_j: MultiPoly,
}

impl Prover {
	pub fn new(g: &MultiPoly) -> Self {
		Prover {
			o_g: g.clone(),
			g_j: g.clone(),
		}
	}

	// pub fn gen_gj(&self) -> MultiPoly {
	// 	(0..n).map(self.evaluate_poly([x, 0, 1, 0, 1]))
	// 	// TODO manual computation.. perhaps can be done recursively...
	// }

	// Prover fixes one more variable with r
	// Returns: a multivar polynomial
	// pub fn fix_polynomial(&self, r: ScalarField) -> UniPoly {
	// 	// modifies self.g_j
	// 	// calls evaluates_gj to get the evaluated univariate polynomial
	// }

	// // Fix 1 variable and evaluate the rest over the points
	// // returns univariable
	// pub fn evaluate_gj(&self, points: Vec<ScalarField>) -> UniPoly {
	// 	let terms: Vec<(ScalarField, SparseTerm)> = cfg_into_iter!(g.terms())
	// 		.map(|(coeff, term)| (*coeff, evaluate_term(&term, &points)))
	// 		.collect();
	// 	// TODO sum the non terms...
	// 	// rebuild it back into a polynomial
	// 	println!("evaluate poly: {:?}", terms);
	// }

	// // returns a term that's evaluated...
	// pub fn evaluate_term(term: &SparseTerm, points: &Vec<ScalarField>) -> SparseTerm {
	// 	let variables_in_term = cfg_into_iter!(term).map(|(var, power)| (var, power));
	// 	println!("evaluate_term: {:?}", variables_in_term);
	// 	// TODO rebuild it back into a single term...
	// }

	// Sum all evaluations of polynomial `g` over boolean hypercube
	pub fn sum_g(&self) -> ScalarField {
		let v = self.o_g.num_vars();
		let n = 2u32.pow(v as u32);
		(0..n)
			.map(|n| self.o_g.evaluate(&n_to_vec(n as usize, v)))
			.sum()
	}
}

// Converts i into an index in {0,1}^v
pub fn n_to_vec(i: usize, n: usize) -> Vec<ScalarField> {
	format!("{:0>width$}", format!("{:b}", i), width = n)
		.chars()
		.map(|x| if x == '1' { 1.into() } else { 0.into() })
		.collect()
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
