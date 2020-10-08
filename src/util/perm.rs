//! Permutation object
use std::vec::Vec;

#[derive(Clone)]
pub struct Permutation<T> {
    index: usize,
    data: Vec<T>,
}

impl<T: Clone + Ord> Permutation<T> {
    /// Create new permutation from a set of objects
    pub fn new(v: &[T]) -> Permutation<T> {
        let mut v = v.to_vec();
        v.sort();
        v.dedup();
        Permutation {
            index: 0usize,
            data: v,
        }
    }
    pub fn index(&self) -> usize {
        self.index
    }
    pub fn data(&self) -> Vec<T> {
        self.data.clone()
    }
}

impl<T: Clone> Iterator for Permutation<T> {
    type Item = Vec<T>;

    // next() is the only required method
    fn next(&mut self) -> Option<Self::Item> {
        // Increment our count. This is why we started at zero.
        self.index += 1;
        todo!()
    }
}

pub trait Generate<T> {
    type Perm;
    fn generate(&self, r: usize) -> Option<Self::Perm>;
    fn count(&self, r: usize) -> Option<usize>;
}

impl<T: Clone + PartialOrd + Ord> Generate<T> for Permutation<T> {
    type Perm = Vec<Vec<T>>;
    /// Lexicographic r-permutation generation, from Knuth The Art of Programming Volume 4A Section 7.2.1.2
    fn generate(&self, r: usize) -> Option<Self::Perm> {
        let mut current = self.data.clone();
        let n = current.len();
        if r > n || n == 0 || r == 0 {
            return None
        }
        let mut ret = Vec::<Vec<T>>::new();
        loop {
            if ret.is_empty() {
                current.sort();
            } else if current[r - 1] < current[n - 1] {
                let mut j = r;
                while current[j] <= current[r - 1] {
                    j += 1;
                }
                current.swap(r - 1, j);
            } else {
                current[r..n].reverse();
                let mut j = r - 1;
                while j > 0 && current[j - 1] >= current[j] {
                    j -= 1;
                }
                if j == 0 {
                    return Some(ret)
                }
                let mut l = n - 1;
                while current[j - 1] >= current[l] {
                    l -= 1;
                }
                current.swap(j - 1, l);
                current[j..n].reverse();
            }
            ret.push(current[0..r].to_vec());
        }
    }
    /// Returns the number of r-permutations for passed in r-value
    fn count(&self, r: usize) -> Option<usize> {
        let n = self.data.len();
        if n == 0 || r == 0 || r > n {
            None
        } else {
            Some(((n - r + 1)..n + 1).product()) // n! / (n - r)!
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
    }
}
