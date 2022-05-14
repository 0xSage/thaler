// Converts i into an index in {0,1}^v
// Index is used to retrieves f evaluations
// Padds to a vector of length, e.g. 000101
pub fn n_to_vec(i: u64, n: usize) -> Vec<bool> {
	let empty: Vec<bool> = vec![false; n];
	let x: Vec<bool> = format!("{:0>width$}", format!("{:b}", i), width = n)
		.chars()
		.map(|x| x == '1')
		.collect();
	// println!("{:?}", x);
	x
}

// Computes Chi_w(r) for any w, O(log n) operations
// w: in {0,1}^v
// r: in {p}^v, e.g. F_5
pub fn chi_w(w: &Vec<bool>, r: &Vec<i128>) -> i128 {
	// assert_eq!(w.len(), r.len());
	// println!("calculating chi_w for w: ");
	// println!("{:?}", w);
	// println!("calculating chi_w for r: ");
	// println!("{:?}", r);

	let product: i128 = w
		.iter()
		.zip(r.iter())
		.map(|(&w, &r)| i128::from(w) * r + (1 - i128::from(w)) * (1 - r))
		.product();
	product
}

// Given a vector `r` in (F_logn)
// Given evaluation table of f(w), for all w in {0,1}^v
// stored in vec, as 000, 001, 010, etc.
// Given n = 2^v
// Given p: the field size, e.g. 5
// Compute f~(r) in O(n log n) time.
// Output: evaluation in field F_p
pub fn stream_mle(fw: &Vec<i128>, r: &Vec<i128>, p: i128) -> i128 {
	assert_eq!(r.len() as f64, (fw.len() as f64).sqrt());
	let sum: i128 = fw
		.iter()
		.enumerate()
		.map(|(i, val)| val * chi_w(&n_to_vec(i as u64, r.len()), r))
		.sum();
	sum % p
}

// Procedure is v stages
// Create chi table, storing w_000, w_001
// Memoizing each iteration, e.g.
// w_0 contributes to calculation of w_00, w_01
// w_1 contributes to calcluation of w_10, 1_11
pub fn dynamic_mle() {}
