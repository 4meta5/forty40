//! Permutation object
use std::vec::Vec;

#[derive(Clone)]
pub struct Permutation<T>(pub Vec<T>);

impl<T: Clone + Ord> Permutation<T> {
    pub fn new(v: &[T]) -> Permutation<T> {
        let mut v = v.to_vec();
        v.sort();
        v.dedup();
        Permutation(v)
    }
}

/// Sequential lexicographic permutations
pub trait PermIter {
    type Item;
    fn prev(&mut self) -> Option<Self::Item>;
    fn next(&mut self) -> Option<Self::Item>;
}

impl<T: Clone + PartialOrd> PermIter for Permutation<T> {
    type Item = Vec<T>;
    fn prev(&mut self) -> Option<Self::Item> {
        if self.0.len() < 2 {
            return None
        }
        // Identify longest, rightmost weakly increasing subslice
        let mut i = self.0.len() - 1;
        while i > 0 && self.0[i - 1] <= self.0[i] {
            i -= 1;
        }
        // Counter is 0 => already in first order spot
        if i == 0 {
            return None
        }
        // Reverse weakly increasing subslice
        self.0[i..].reverse();
        // Find rightmost element >= pivot at i - 1 spot
        let mut j = self.0.len() - 1;
        while j >= i && self.0[j - 1] < self.0[i - 1] {
            j -= 1;
        }
        // Swap element with element at pivot position
        self.0.swap(i - 1, j);
        Some(self.0.clone())
    }
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.len() < 2usize {
            return None
        }
        // Get index range of longest, rightmost weakly decreasing subslice
        let mut i = self.0.len() - 1;
        while i > 0 && self.0[i - 1] >= self.0[i] {
            i -= 1;
        }
        // Counter is 0 => Entire vector => last-ordered permutation.
        if i == 0 {
            return None
        }
        // Find rightmost element larger than pivot <=> i - 1
        let mut j = self.0.len() - 1;
        while j >= i && self.0[j] <= self.0[i - 1] {
            j -= 1;
        }
        // Swap element with the pivot
        self.0.swap(j, i - 1);
        // Reverse last weakly decreasing part
        self.0[i..].reverse();
        Some(self.0.clone())
    }
}

pub trait GenerateR<T> {
    type Perm;
    fn generate(&self, r: usize) -> Option<Self::Perm>;
    fn count_perms(&self, r: usize) -> Option<usize>;
}

impl<T: Clone + PartialOrd + Ord> GenerateR<T> for Permutation<T> {
    type Perm = Vec<Vec<T>>;
    /// Lexicographic R-Permutation Generation with Restrictions
    /// by Knuth Vol 4A Sec 7.2.1.2 The Art of Programming
    fn generate(&self, sub: usize) -> Option<Self::Perm> {
        let mut vec = self.0.clone();
        let size = vec.len();
        // -> TODO: add constraint n! / (n - r)! >= 16!
        let (above_max, under_min) = (sub == size && size > 16usize, size < 2);
        if sub > size || sub == 0 || above_max || under_min {
            return None
        }
        let mut ret = Vec::<Vec<T>>::new();
        loop {
            if ret.is_empty() {
                vec.sort();
            } else if vec[sub - 1] < vec[size - 1] {
                let mut counter = sub;
                while vec[counter] <= vec[sub - 1] {
                    counter += 1;
                }
                vec.swap(sub - 1, counter);
            } else {
                vec[sub..size].reverse();
                let mut counter = sub - 1;
                while counter > 0 && vec[counter - 1] >= vec[counter] {
                    counter -= 1;
                }
                if counter == 0 {
                    return Some(ret)
                }
                let mut lc = size - 1;
                while vec[counter - 1] >= vec[lc] {
                    lc -= 1;
                }
                vec.swap(counter - 1, lc);
                vec[counter..size].reverse();
            }
            ret.push(vec[0..sub].to_vec());
        }
    }
    /// Returns the number of r-permutations for passed in r-value
    fn count_perms(&self, sub: usize) -> Option<usize> {
        let size = self.0.len();
        if size < 2 || sub == 0 || sub > size {
            None
        } else {
            Some(((size - sub + 1)..size + 1).product()) // n! / (n - r)!
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
    #[test]
    fn iter_works() {
        let v = vec![1, 2, 3, 4, 5];
        let mut p = Permutation::new(v.as_slice());
        let mut j = 1usize;
        while j < 120 {
            assert!(p.next().is_some());
            j += 1;
        }
        assert!(p.next().is_none());
        while j > 1 {
            assert!(p.prev().is_some());
            j -= 1;
        }
        assert!(p.prev().is_none());
    }
}
