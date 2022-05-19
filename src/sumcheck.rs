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

// Public helper fns for prover
// Evaluates a term with a fixed var, returning (new coefficent, fixed term)
pub fn evaluate_term(
	term: &SparseTerm,
	point: &Vec<ScalarField>,
) -> (ScalarField, Option<SparseTerm>) {
	println!("term: {:?}", term);
	let mut fixed_term: Option<SparseTerm> = None;
	let coeff: ScalarField =
		cfg_into_iter!(term).fold(1u32.into(), |product, (var, power)| match *var {
			0 => {
				fixed_term = Some(SparseTerm::new(vec![(*var, *power)]));
				product
			}
			_ => point[*var].pow(&[*power as u64]) * product,
		});
	println!("evaluate term product: {:?}", coeff.into_repr());
	println!("evaluate term fixed term: {:?}", fixed_term);
	(coeff, fixed_term)
}

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
			o_g: g.clone(), // unmodified, can be just a static ref
			g_j: g.clone(), // modified each time
		}
	}

	// kicks off computing each permutation of boolean hypercube...
	// pub fn gen_gj(&self) -> UniPoly {
	// 	(0..n).map(self.evaluate_poly([x, 0, 1, 0, 1]))
	// }

	// Prover fixes one more variable with r
	// Returns: a univariate polynomial
	pub fn fix_polynomial(&mut self, r: Option<ScalarField>) {
		// 1. self modify g_j, into g(r... Xj, x, x), skip in 1st step

		// 2. (0..n).map(self.evaluate_poly([(redacted), 0, 0, 0, etc]))
		// calls evaluates_gj to get the evaluated univariate polynomial

		// for testing
		let gj = self.evaluate_gj(vec![1.into(), 1.into(), 1.into()]);
		println!("fixed polynomial is now: {:?}", gj);
	}

	// evaluates g_j over points
	// returns univariate::Polynomial with x_0 fixed
	pub fn evaluate_gj(&self, points: Vec<ScalarField>) -> UniPoly {
		// term coefficient
		let unipoly_coefficients: Vec<(usize, ScalarField)> = cfg_into_iter!(self.g_j.terms())
			.map(|(coeff, term)| {
				let (coeff_eval, fixed_term) = evaluate_term(&term, &points);
				// fixed_term is Option<SparseTerm>
				match fixed_term {
					// (degree, coefficient)
					None => (0, *coeff * coeff_eval),
					_ => (fixed_term.unwrap().degree(), *coeff * coeff_eval),
				}
			})
			.collect();
		// Note: 0th degree is the constant...
		println!("unipoly coefficients: {:?}", unipoly_coefficients);
		UniPoly::from_coefficients_vec(unipoly_coefficients)
	}

	// Sum all evaluations of polynomial `g` over boolean hypercube
	pub fn sum_g(&self) -> ScalarField {
		let v = self.o_g.num_vars();
		let n = 2u32.pow(v as u32);
		(0..n)
			.map(|n| self.o_g.evaluate(&n_to_vec(n as usize, v)))
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
