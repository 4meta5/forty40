//! Lexicographic r-permutation generation
use std::hash::Hash;

pub struct Lex<T> {
    pub mset: Vec<T>,
    // r
    size: usize,
}

impl<T: Ord> Lex<T> {
    pub fn new(mut mset: Vec<T>, size: usize) -> Self {
        mset.sort();
        Self { mset, size }
    }
}

impl<T: Clone + Ord + Hash + PartialEq> Iterator for Lex<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        let n = self.mset.len();
        let r = self.size;
        if n == 0 || r == 0 || r > n {
            return None;
        }
        if self.mset[r - 1] < self.mset[n - 1] {
            let mut j = r;
            while self.mset[j] <= self.mset[r - 1] {
                j += 1;
            }
            self.mset.swap(r - 1, j);
        } else {
            self.mset[r..n].reverse();
            let mut j = r - 1;
            while j > 0 && self.mset[j - 1] >= self.mset[j] {
                j -= 1;
            }
            if j == 0 {
                return None;
            }
            let mut l = n - 1;
            while self.mset[j - 1] >= self.mset[l] {
                l -= 1;
            }
            self.mset.swap(j - 1, l);
            self.mset[j..n].reverse();
        }
        Some(self.mset[0..r].to_vec())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.mset.is_empty() || self.size == 0 || self.size > self.mset.len() {
            (0, Some(0))
        } else {
            (
                1,
                Some(((self.size - self.mset.len() + 1)..=self.size).product()),
            )
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn tuple_size_check() {
        let raw_vec = vec![1, 3, 5, 7];
        let one = Lex::new(raw_vec.clone(), 1);
        let two = Lex::new(raw_vec.clone(), 2);
        // let three = Lex::new(raw_vec.clone(), 3);
        // let four = Lex::new(raw_vec.clone(), 4);
        assert_eq!(one.size, two.size - 1usize);
        // assert_eq!(three.size_hint(), (1, Some(24)));
        // assert_eq!(four.size_hint(), (1, Some(24)));
    }
}
