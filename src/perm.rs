//! Multipermutations
use std::vec::Vec;

#[derive(Clone)]
pub struct Permutation<T>(Vec<T>);

impl<T: Clone + Ord> Permutation<T> {
    pub fn new(v: &[T]) -> Permutation<T> {
        Permutation(v.to_vec())
    }
    fn size_hint(&self, r: usize) -> (usize, Option<usize>) {
        let n = self.0.len();
        if n == 0 || r == 0 || r > n {
            (0, Some(0))
        } else {
            (1, Some(((n - r + 1)..n + 1).product())) // n! / (n - r)!
        }
    }
    /// Heaps Algorithm for generating all r-permutations
    pub fn generate(&mut self, r: usize) -> Option<Vec<Vec<T>>> {
        let n = self.0.len();
        if r > n || n == 0 || r == 0 {
            return None
        }
        // TODO: enforce bound on `r` (subset size) here, maybe 16usize
        let mut stack = vec![0usize; self.0.len()];
        let mut ret = Vec::<Vec<T>>::new();
        let mut i = 0usize;
        while i < r {
            if stack[i] < i {
                if r % 2 == 0 {
                    self.0.swap(i, r - 1);
                } else {
                    self.0.swap(0, r - 1);
                }
                stack[i] += 1usize;
                i = 0usize;
                ret.push(self.0.clone());
            } else {
                stack[i] = 0usize;
                i += 1usize;
            }
        }
        Some(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::vec;
    #[test]
    fn generates_permutations() {
        let v = vec![1, 2, 3, 4, 5];
        let mut p = Permutation::new(v.as_slice());
        assert!(p.size_hint(3) == (1usize, Some(60)));
    }
}
