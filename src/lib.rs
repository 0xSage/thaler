//! This Freivald library unsafely implements Freivald's algorithm
//! It does not handle overflows, and merely continues execution

#![allow(arithmetic_overflow)]
use ndarray::Array2;

pub fn freivald_verify(a: &Array2<u128>, b: &Array2<u128>, c: &Array2<u128>) -> bool {
    // @dev: implement this
    unimplemented!()
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
