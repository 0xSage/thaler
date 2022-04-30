//! The Freivald crate implements Freivald's algorithm

use ndarray::{arr1, arr2, Array2};

pub fn freivald_verify() -> bool {
    unimplemented!()
}

pub fn dumb_verify(a: &Array2<i64>, b: &Array2<i64>, c: &Array2<i64>) -> bool {
    a.dot(b) == c
}
