//! The Freivald crate implements Freivald's algorithm

use ndarray::{arr1, arr2, Array1, Array2};
use std::iter::Iterator;
struct Univar {
    product: u128,
    r: u128,
}

impl Univar {
    fn new(r: u128) -> Self {
        Univar { product: 1, r }
    }
}

impl Iterator for Univar {
    type Item = u128;
    fn next(&mut self) -> Option<Self::Item> {
        self.product *= self.r;
        Some(self.product)
    }
}

pub fn get_r() -> u128 {
    5
}

pub fn get_vec(r: u128, n: usize) -> Array1<u128> {
    let var = Univar::new(r);
    var.take(n).collect()
}

pub fn freivald_verify(a: &Array2<u128>, b: &Array2<u128>, c: &Array2<u128>) -> bool {
    assert!(check_matrix_dimensions(a, b, c));
    let v = get_vec(get_r(), c.ncols());
    a.dot(&b.dot(&v)) == c.dot(&v)
}

pub fn dumb_verify(a: &Array2<u128>, b: &Array2<u128>, c: &Array2<u128>) -> bool {
    assert!(check_matrix_dimensions(a, b, c));
    a.dot(b) == c
}

pub fn check_matrix_dimensions(a: &Array2<u128>, b: &Array2<u128>, c: &Array2<u128>) -> bool {
    a.nrows() == b.ncols()
        && a.ncols() == b.nrows()
        && c.nrows() == a.nrows()
        && c.ncols() == b.ncols()
}
