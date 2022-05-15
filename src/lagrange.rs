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

// Computes Chi_w(r) for all w, O(log n) operations
pub fn chi_w(w: &Vec<bool>, r: &Vec<i128>) -> i128 {
	assert_eq!(w.len(), r.len());
	let product: i128 = w
		.iter()
		.zip(r.iter())
		.map(|(&w, &r)| chi_step(w, r))
		.product();
	product
}

// Calculating the slow way, for benchmarking
pub fn slow_mle(fw: &Vec<i128>, r: &Vec<i128>, p: i128) -> i128 {
	assert_eq!(r.len() as f64, (fw.len() as f64).sqrt());
	let sum: i128 = fw
		.iter()
		.enumerate()
		.map(|(i, val)| val * chi_w(&n_to_vec(i, r.len()), r))
		.sum();
	sum % p
}

// Lemma 3.7
pub fn stream_mle(fw: &Vec<i128>, r: &Vec<i128>, p: i128) -> i128 {
	recurse(fw, r, 2usize.pow(r.len() as u32)) % p
}

pub fn recurse(fw: &Vec<i128>, r: &Vec<i128>, n: usize) -> i128 {
	match n {
		0 => 0,
		_ => recurse(fw, r, n - 1) + fw[n - 1] * chi_w(&n_to_vec(n - 1, r.len()), r),
	}
}

// Lemm 3.8
pub fn dynamic_mle(fw: &Vec<i128>, r: &Vec<i128>, p: i128) -> i128 {
	let chi_lookup = memoize(r, r.len());
	let result: i128 = fw
		.iter()
		.zip(chi_lookup.iter())
		.map(|(left, right)| left * right)
		.sum();
	result % p
}

pub fn memoize(r: &Vec<i128>, v: usize) -> Vec<i128> {
	match v {
		1 => {
			vec![chi_step(false, r[v - 1]), chi_step(true, r[v - 1])]
		}
		_ => memoize(r, v - 1)
			.iter()
			.flat_map(|val| {
				[
					val * chi_step(false, r[v - 1]),
					val * chi_step(true, r[v - 1]),
				]
			})
			.collect(),
	}
}
