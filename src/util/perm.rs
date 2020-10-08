//! Permutation object
use std::vec::Vec;

#[derive(Clone)]
pub struct Permutation<T> {
    ptr: usize,
    data: Vec<T>,
}

impl<T: Clone + Ord> Permutation<T> {
    pub fn new(v: &[T]) -> Permutation<T> {
        let mut v = v.to_vec();
        v.sort();
        v.dedup();
        Permutation {
            ptr: 1usize,
            data: v,
        }
    }
    pub fn ptr(&self) -> usize {
        self.ptr
    }
    pub fn data(&self) -> Vec<T> {
        self.data.clone()
    }
}

impl<T: Clone + PartialOrd> Iterator for Permutation<T> {
    type Item = Vec<T>;
    /// Sequential permutation generation
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

pub trait GenerateR<T> {
    type Perm;
    fn generate(&self, r: usize) -> Option<Self::Perm>;
    fn count_perms(&self, r: usize) -> Option<usize>;
}

impl<T: Clone + PartialOrd + Ord> GenerateR<T> for Permutation<T> {
    type Perm = Vec<Vec<T>>;
    /// Knuth Algorithm X for Permutation Generation, Vol 4A Sec 7.2.1.2
    fn generate(&self, r: usize) -> Option<Self::Perm> {
        let mut current = self.data.clone();
        let n = current.len();
        // limit to prevent permutation generation for anything greater than 16!
        // -> TODO: similar limit for when n! / (n - r)! >= 16!
        let comp_limit = r == n && n > 16usize;
        if r > n || n == 0 || r == 0 || comp_limit {
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
    fn count_perms(&self, r: usize) -> Option<usize> {
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
        let p = Permutation::new(v.as_slice());
        assert!(p.count_perms(3) == Some(60));
        assert!(p.generate(5).unwrap().len() == 120);
    }
}
