//! Number Theory Helpers
#![allow(dead_code)]
/// Sum of first n natural numbers
fn n_sum(n: usize) -> usize {
    (n * (n + 1)) / 2
}

/// Sum of n^{2} sequence up to nth value
fn n_squared_sum(n: usize) -> usize {
    (n * (n + 1) * ((2 * n) + 1)) / 6
}

/// Sum of n^{3} sequence up to nth value
fn n_cubed_sum(n: usize) -> usize {
    ((n * n) * ((n + 1) * (n + 1))) / 4
}

/// Euler's constant
/// e = lim (1 + 1/n) as n approaches infinity
fn euler_constant(n: usize) -> usize {
    1 / fac(n)
}

/// Factorial function for calculating n! for some n
pub fn fac(n: usize) -> usize {
    (1..n + 1).product()
}
