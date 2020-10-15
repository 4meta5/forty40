//! Counting

/// Factorial function for calculating n! for some n
#[inline]
pub fn fac(n: usize) -> usize {
    (1..n + 1).product()
}

/// Order matters
/// n! / (n - k)! = n * (n - 1) * ... * (n - k + 1)
#[inline]
fn n_perm_k(n: usize, k: usize) -> usize {
    ((n - k + 1)..n + 1).product()
}

#[inline]
/// Binomial coefficient -- order doesn't matter
/// n! / k!(n - k)! = n * (n - 1) * ... * (n - k + 1) / k!
fn n_choose_k(n: usize, k: usize) -> usize {
    n_perm_k(n, k) / fac(k)
}

/// Binomial theorem
/// (x + y) ^ {n} = \sum_{k = 0}{n} (n choose k) x^{n - k}{y^k}
pub fn binomial_thm(x: usize, y: usize, n: usize) -> usize {
    let mut ret = 0usize;
    for k in 0..n {
        let val = n_choose_k(n, k) * x.pow((n - k) as u32) * y.pow(k as u32);
        ret += val;
    }
    ret
}

/// Combination with repetition
/// (n + k - 1) choose k
pub fn n_multichoose_k(n: usize, k: usize) -> usize {
    n_choose_k(n + k - 1, k)
}

// Theorem 2.4.3
// Let n be a positive integer and let n_{1}, .., n_{k} be positive integers s.t. n = n_{1} + ... + n_{k}. The number of ways to partition a set of n objects into k labeled boxes in which Box 1 contains n_{1} objects, Box 2 contains n_{2} objects, ..., Box k contains n_{k} objects equals n! / (n_{1}! * n_{2}! * ... * n_{k}!)
//
// If the boxes are not labeled and n_{1} = ... = n_{k}, then the number of partitions equals n! / (k!n_{1}!n_{2}! * ... * n_{k}!)
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
    fn perm_tests() {
        assert_eq!(n_perm_k(5, 2), 20);
        assert_eq!(n_perm_k(2, 2), 2);
    }
    #[test]
    fn comb_tests() {
        assert_eq!(n_choose_k(5, 2), 10);
        assert_eq!(n_choose_k(2, 2), 1);
    }
    #[test]
    fn multichoose_tests() {
        assert_eq!(n_multichoose_k(6, 3), 56);
        assert_eq!(n_multichoose_k(8, 4), 330);
    }
}
