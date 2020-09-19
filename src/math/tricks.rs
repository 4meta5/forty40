//! Number Theory Tricks
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn n_sum_works() {
        assert_eq!(n_sum(100), 5050);
        assert_eq!(n_sum(5), 15);
        assert_eq!(n_sum(6), 21);
        assert_eq!(n_sum(7), 28);
        assert_eq!(n_sum(8), 36);
        assert_eq!(n_sum(9), 45);
        assert_eq!(n_sum(10), 55);
    }
    #[test]
    fn n_squared_sum_works() {
        assert_eq!(n_squared_sum(5), 55);
        assert_eq!(n_squared_sum(6), 91);
        assert_eq!(n_squared_sum(7), 140);
        assert_eq!(n_squared_sum(8), 204);
        assert_eq!(n_squared_sum(9), 285);
        assert_eq!(n_squared_sum(10), 385);
    }
    #[test]
    fn n_cubed_sum_works() {
        assert_eq!(n_cubed_sum(5), 225);
        assert_eq!(n_cubed_sum(6), 441);
        assert_eq!(n_cubed_sum(7), 784);
        assert_eq!(n_cubed_sum(8), 1296);
        assert_eq!(n_cubed_sum(9), 2025);
        assert_eq!(n_cubed_sum(10), 3025);
    }
}
