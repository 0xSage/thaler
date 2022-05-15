// TODO use Fr
use std::collections::HashMap;

// One step in chi
pub fn chi_step(w: bool, x: i128) -> i128 {
	x * i128::from(w) + (1 - x) * (1 - i128::from(w))
}

// Converts i into an index in {0,1}^v
// Index is used to retrieves f evaluations
// Pads to a vector of length, e.g. 000101
pub fn n_to_vec(i: usize, n: usize) -> Vec<bool> {
	let x: Vec<bool> = format!("{:0>width$}", format!("{:b}", i), width = n)
		.chars()
		.map(|x| x == '1')
		.collect();
	x
}

// Computes Chi_w(r) for any w, O(log n) operations
// w: in {0,1}^v
// r: in {p}^v, e.g. F_5
pub fn chi_w(w: &Vec<bool>, r: &Vec<i128>) -> i128 {
	assert_eq!(w.len(), r.len());
	let product: i128 = w
		.iter()
		.zip(r.iter())
		.map(|(&w, &r)| chi_step(w, r))
		.product();
	product
}

// Given a vector `r` in (F_logn)
// Given evaluation table of f(w), for all w in {0,1}^v
// Stored in vec, as 000, 001, 010, etc.
// Given n = 2^v
// Given p: the field size, e.g. 5
// Compute f~(r) in linear (slow) way
// Output: evaluation in field F_p
pub fn slow_mle(fw: &Vec<i128>, r: &Vec<i128>, p: i128) -> i128 {
	assert_eq!(r.len() as f64, (fw.len() as f64).sqrt());
	let sum: i128 = fw
		.iter()
		.enumerate()
		.map(|(i, val)| val * chi_w(&n_to_vec(i, r.len()), r))
		.sum();
	sum % p
}

// --------------------
// Similar to slow_mle, but recursive:
// f~(r) <- f~(r) + f(w) * chi_w(r)
// Brings improvements in memory usage, storing just O(log n) elements
pub fn stream_mle(fw: &Vec<i128>, r: &Vec<i128>, p: i128) -> i128 {
	recurse(fw, r, 2usize.pow(r.len() as u32)) % p
}

pub fn recurse(fw: &Vec<i128>, r: &Vec<i128>, n: usize) -> i128 {
	match n {
		0 => 0,
		_ => recurse(fw, r, n - 1) + fw[n - 1] * chi_w(&n_to_vec(n - 1, r.len()), r),
	}
}

// --------------------

// Procedure is v stages
// Create chi table, storing w_000, w_001, w_010, so on.
// Memoizing each iteration, e.g.
// w_0 contributes to calculation of w_00, w_01
// w_1 contributes to calcluation of w_10, 1_11, so on
pub fn dynamic_mle(fw: &Vec<i128>, r: &Vec<i128>, p: i128) -> i128 {
	let chi_lookup = memoize(r, r.len()); // vec of length n = 2^v

	let result: i128 = fw
		.iter()
		.zip(chi_lookup.iter())
		.map(|(left, right)| left * right)
		.sum();
	result % p
	// Step 2: get inner product of f(w) and chi(r), i.e. just fw dot lookup table.
}

// v=0, recurse & implicitly "memoize" the redundant calculations
// track w as a string..."00" "000" are different...
pub fn memoize(
	r: &Vec<i128>,
	v: usize, // start 2
) -> Vec<i128> {
	match v {
		1 => {
			println!("Matched v=1 (once)");
			let basevec = vec![chi_step(false, r[v - 1]), chi_step(true, r[v - 1])];
			println!("Base vec: {:?}", basevec);
			basevec
		}
		_ => {
			println!("Matched v={:?}", v);
			// or just get the prev vec and flatten it... vec![prev vec.flatten]
			// each vector of previous vec* chim, prev_vec * chi.
			// explode vector out into bigger vec...
			let vec0: Vec<i128> = memoize(r, v - 1)
				.iter()
				.flat_map(|val| {
					[
						val * chi_step(false, r[v - 1]),
						val * chi_step(true, r[v - 1]),
					]
				})
				.collect();
			println!("Vec now looks like: {:?} (once)", vec0);
			vec0
		}
	}
}
