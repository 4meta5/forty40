//! [src](https://github.com/janmarthedal/snippets/blob/master/rust/generate/permutations/src/lib.rs)
use crate::MultiSet;

pub struct Permutations<T> {
    pub set: MultiSet<T>,
    size: usize,
}

impl<T> Permutations<T> {
    pub fn new(set: MultiSet<T>, size: usize) -> Self {
        Self { set, size }
    }
}

impl<T: Clone + Ord> Iterator for Permutations<T> {
    type Item = MultiSet<T>;

    fn next(&mut self) -> Option<MultiSet<T>> {
        let n = self.set.0.len();
        let r = self.size;
        if n == 0 || r == 0 || r > n {
            return None;
        }
        if self.set.0[r - 1] < self.set.0[n - 1] {
            let mut j = r;
            while self.set.0[j] <= self.set.0[r - 1] {
                j += 1;
            }
            self.set.0.swap(r - 1, j);
        } else {
            self.set.0[r..n].reverse();
            let mut j = r - 1;
            while j > 0 && self.set.0[j - 1] >= self.set.0[j] {
                j -= 1;
            }
            if j == 0 {
                return None;
            }
            let mut l = n - 1;
            while self.set.0[j - 1] >= self.set.0[l] {
                l -= 1;
            }
            self.set.0.swap(j - 1, l);
            self.set.0[j..n].reverse();
        }
        Some(MultiSet::from(self.set.0[0..r].to_vec()))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.set.0.len();
        let r = self.size;
        if n == 0 || r == 0 || r > n {
            (0, Some(0))
        } else {
            (1, Some(((n - r + 1)..=n).product()))
        }
    }
}

pub fn permutations<T: Clone + Ord>(s: &[T], size: usize) -> Permutations<T> {
    let vec = s.to_vec();
    Permutations {
        set: MultiSet::from(vec),
        size,
    }
}

#[cfg(test)]
mod test {
    use crate::{MultiSet, Permutations};
    #[test]
    fn tuple_size_check() {
        let raw_vec = vec![1, 3, 3, 3, 5, 5, 5, 5, 5, 6, 7, 7, 7, 7, 7, 7];
        let set = MultiSet::from(raw_vec.clone());
        let set_cpy = MultiSet::from(raw_vec);
        let three_permutations = Permutations::new(set, 3);
        let four_permutations = Permutations::new(set_cpy, 4);
        assert_eq!(three_permutations.size, four_permutations.size - 1usize);
        assert_eq!(three_permutations.size_hint(), (1, Some(3360)));
        assert_eq!(four_permutations.size_hint(), (1, Some(43680)));
    }
}
