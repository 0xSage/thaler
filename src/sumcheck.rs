use ark_bls12_381::Fr as ScalarField;
use ark_ff::biginteger::BigInteger256;
use ark_ff::{BigInteger, Field, FpParameters, PrimeField};
use ark_poly::polynomial::multivariate::{SparsePolynomial, SparseTerm, Term};
use ark_poly::polynomial::{MVPolynomial, Polynomial};

// Polynomials representations:
type MultiPoly = SparsePolynomial<ScalarField, SparseTerm>;

// Prover
// Prover: generates univariate polynomial g_j(X_j) where j is the variable/coordinate being fixed
// Super inefficient
// Notice prover side only becomes efficient when we do multilinear extension of g...
pub fn gen_gj() {
	// TODO manual computation.. perhaps can be done recursively...
}

// Converts i into an index in {0,1}^v
pub fn n_to_vec(i: usize, n: usize) -> Vec<ScalarField> {
	let x: Vec<ScalarField> = format!("{:0>width$}", format!("{:b}", i), width = n)
		.chars()
		.map(|x| if x == '1' { 1.into() } else { 0.into() })
		.collect();
	x
}

// Sum all evaluations of a polynomial g over a boolean hypercube
pub fn sum_g(g: &MultiPoly) -> ScalarField {
	let v: u32 = g.num_vars() as u32;
	let n = 2u32.pow(v);
	let sum: ScalarField = (0..n)
		.map(|n| g.evaluate(&n_to_vec(n as usize, g.num_vars())))
		.sum();
	sum
}

// Verifier

// Verifier: Random r over large field F
pub fn get_r() -> i128 {
	// TODO implement this
	1.into()
}

// Verifier: Evaluates univariate polynomial g at x
pub fn eval_gx(x: i128, g: Vec<i128>) -> i128 {
	0
}

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
