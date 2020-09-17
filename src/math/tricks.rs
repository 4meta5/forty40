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

/// Euler's Constant Approximation
/// e = lim (1 + 1/n)^{n} as n approaches infinity
/// Returns (a, b) s.t. a/b is an approximation of e that improves with greater n
fn euler_c(n: usize) -> (usize, usize) {
    ((n + 1).pow(n as u32), n.pow(n as u32))
}

/// Bernoulli Number Approximation
/// -> 2 should be replaced with e (in body)
fn bernoulli_num(c: usize) -> (usize, usize) {
    (c, 2usize.pow(c as u32) - 1)
}

/// Factorial function for calculating n! for some n
pub fn fac(n: usize) -> usize {
    (1..n + 1).product()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn fac_tests() {
        assert_eq!(fac(5), 120);
        assert_eq!(fac(6), 720);
        assert_eq!(fac(7), 5040);
        assert_eq!(fac(8), 40320);
        assert_eq!(fac(9), 362880);
        assert_eq!(fac(10), 3628800);
    }
    #[test]
    fn test_e() {
        assert_eq!((2, 1), euler_c(1));
        assert_eq!((9, 4), euler_c(2));
        assert_eq!((64, 27), euler_c(3));
    }
}
