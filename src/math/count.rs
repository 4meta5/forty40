//! Combinatorics
#![allow(dead_code)]

/// Factorial function for calculating n! for some n
#[inline]
pub fn fac(n: usize) -> usize {
    (1..n + 1).product()
}

/// Order matters
/// n! / (n - r)! = n * (n - 1) * ... * (n - r + 1)
#[inline]
fn permute(n: usize, r: usize) -> usize {
    ((n - r + 1)..n + 1).product()
}

/// Order does not matter
/// n!/r!(n-r)!
fn binomial_co(n: usize, r: usize) -> usize {
    let if_order_mattered = permute(n, r);
    if_order_mattered / fac(r)
}

/// Binomial theorem
/// (x + y) ^ {n} = \sum_{k = 0}{n} (n choose k) x^{n - k}{y^k}
/// TODO: would be more useful to request the nth term in the binomial sequence
pub fn binomial_thm(x: usize, y: usize, n: usize) -> usize {
    let mut ret = 0usize;
    for k in 0..n {
        let val = binomial_co(n, k) * x.pow((n - k) as u32) * y.pow(k as u32);
        ret += val;
    }
    ret
}

/// Permutation
/// if repetition { n choices every time, r times => n^r }
/// else { n!/(n-r)! }
pub fn perm(n: usize, r: usize, repetition: bool) -> usize {
    if repetition {
        n.pow(r as u32)
    } else {
        permute(n, r)
    }
}

/// Combination, order doesn't matter
/// if repetition { (r + n - 1)! / r!(n-1)! }
/// else { n!/r!(n-r)! }
pub fn comb(n: usize, r: usize, repetition: bool) -> usize {
    if repetition {
        let (num, mut dom) = (fac(r + n - 1), fac(n - 1));
        dom *= fac(r);
        num / dom
    } else {
        binomial_co(n, r)
    }
}

/*
Theorem 2.4.3
Let n be a positive integer and let n_{1}, .., n_{k} be positive integers s.t. n = n_{1} + ... + n_{k}. The number of ways to partition a set of n objects into k labeled boxes in which Box 1 contains n_{1} objects, Box 2 contains n_{2} objects, ..., Box k contains n_{k} objects equals n! / (n_{1}! * n_{2}! * ... * n_{k}!)

If the boxes are not labeled and n_{1} = ... = n_{k}, then the number of partitions equals n! / (k!n_{1}!n_{2}! * ... * n_{k}!)
*/
pub fn partition_count<T: PartialEq + Clone + Eq>(
    v: &[T],
    boxes: &[usize],
    labeled: bool,
) -> usize {
    let (vec, boxes) = (v.to_vec(), boxes.to_vec());
    let num = fac(vec.len());
    let mut dom = 1usize;
    if !labeled {
        dom *= fac(boxes.len());
    }
    boxes.into_iter().for_each(|space| dom *= fac(space));
    num / dom
}

#[cfg(test)]
mod test {
    use super::*;
    use alloc::vec;
    fn fac_tests() {
        assert_eq!(fac(5), 120);
        assert_eq!(fac(6), 720);
        assert_eq!(fac(7), 5040);
        assert_eq!(fac(8), 40320);
        assert_eq!(fac(9), 362880);
        assert_eq!(fac(10), 3628800);
    }
    #[test]
    fn perm_tests() {
        assert_eq!(perm(5, 2, true), 25);
        assert_eq!(perm(2, 2, true), 4);
        assert_eq!(perm(5, 2, false), 20);
    }
    #[test]
    fn comb_tests() {
        assert_eq!(comb(5, 2, true), 15);
        assert_eq!(comb(2, 2, true), 3);
        assert_eq!(comb(5, 2, false), 10);
    }
    #[test]
    fn partition_count_tests() {
        let things = vec![1, 3, 4, 5, 7, 10];
        let boxes = vec![2usize, 3usize];
        //  6!/2!3! = 60
        assert_eq!(
            partition_count(things.as_slice(), boxes.as_slice(), true),
            60
        );
        // 60 / k! == 2! => 30
        assert_eq!(
            partition_count(things.as_slice(), boxes.as_slice(), false),
            30
        );
    }
}
