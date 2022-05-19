use ark_bls12_381::Fr as ScalarField;
use ark_ff::biginteger::BigInteger256;
use ark_ff::{BigInteger, Field, FpParameters, PrimeField};
use ark_poly::polynomial::multivariate::{SparsePolynomial, SparseTerm, Term};
use ark_poly::polynomial::{MVPolynomial, Polynomial};

// Polynomials representations:
type MultiPoly = SparsePolynomial<ScalarField, SparseTerm>;

// Super inefficient Prover: generates univariate polynomial g_j(X_j) where j is the variable/coordinate being fixed
// Notice prover side only becomes efficient when we do multilinear extension of g...
pub fn gen_gj() {
	// TODO manual computation.. perhaps can be done recursively...
}

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
