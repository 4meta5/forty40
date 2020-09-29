//! Multipermutations
use std::vec::Vec;

#[derive(Clone)]
pub struct Permutation<T>(Vec<T>);

impl<T: Clone + Ord> Permutation<T> {
    pub fn new(v: &[T]) -> Permutation<T> {
        Permutation(v.to_vec())
    }
    fn count(&self, r: usize) -> Option<usize> {
        let n = self.0.len();
        if n == 0 || r == 0 || r > n {
            None
        } else {
            Some(((n - r + 1)..n + 1).product()) // n! / (n - r)!
        }
    }
    /// Lexicographic r-permutation generation, from Knuth The Art of Programming Volume 4A Section 7.2.1.2
    pub fn generate(&mut self, r: usize) -> Option<Vec<Vec<T>>> {
        let n = self.0.len();
        if r > n || n == 0 || r == 0 {
            return None
        }
        let mut ret = Vec::<Vec<T>>::new();
        loop {
            if ret.is_empty() {
                self.0.sort();
            } else if self.0[r - 1] < self.0[n - 1] {
                let mut j = r;
                while self.0[j] <= self.0[r - 1] {
                    j += 1;
                }
                self.0.swap(r - 1, j);
            } else {
                self.0[r..n].reverse();
                let mut j = r - 1;
                while j > 0 && self.0[j - 1] >= self.0[j] {
                    j -= 1;
                }
                if j == 0 {
                    return Some(ret)
                }
                let mut l = n - 1;
                while self.0[j - 1] >= self.0[l] {
                    l -= 1;
                }
                self.0.swap(j - 1, l);
                self.0[j..n].reverse();
            }
            ret.push(self.0[0..r].to_vec());
        }
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
        assert!(p.count(3) == Some(60));
        assert!(p.generate(3).unwrap().len() == 60);
    } // TODO: example with duplicates, how does it deal with these by default?
}
