use ark_bls12_381::Fr;
use ndarray::{Array1, Array2};
use rand::Rng;
use std::iter::Iterator;

struct Univar {
    product: Fr,
    r: Fr,
}

impl Univar {
    fn new(r: Fr) -> Self {
        Univar {
            product: 1.into(),
            r,
        }
    }
}

impl Iterator for Univar {
    type Item = Fr;
    fn next(&mut self) -> Option<Self::Item> {
        self.product *= self.r;
        Some(self.product)
    }
}

pub fn get_r() -> Fr {
    let mut rng = rand::thread_rng();
    let r: Fr = rng.gen();
    r
}

pub fn get_vec(r: Fr, n: usize) -> Array1<Fr> {
    let var = Univar::new(r);
    var.take(n).collect()
}

pub fn freivald_verify(a: &Array2<Fr>, b: &Array2<Fr>, c: &Array2<Fr>) -> bool {
    assert!(check_matrix_dimensions(a, b, c));
    let v = get_vec(get_r(), c.ncols());
    a.dot(&b.dot(&v)) == c.dot(&v)
}

pub fn dumb_verify(a: &Array2<Fr>, b: &Array2<Fr>, c: &Array2<Fr>) -> bool {
    assert!(check_matrix_dimensions(a, b, c));
    a.dot(b) == c
}

pub fn check_matrix_dimensions(a: &Array2<Fr>, b: &Array2<Fr>, c: &Array2<Fr>) -> bool {
    a.nrows() == b.ncols()
        && a.ncols() == b.nrows()
        && c.nrows() == a.nrows()
        && c.ncols() == b.ncols()
}
