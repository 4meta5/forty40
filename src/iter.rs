//! [src](https://github.com/janmarthedal/snippets/blob/master/rust/generate/permutations/src/lib.rs)
pub struct Permutations<T> {
    pub mset: Vec<T>,
    size: usize,
}

impl<T: Ord> Permutations<T> {
    pub fn new(mut mset: Vec<T>, size: usize) -> Self {
        mset.sort();
        Self { mset, size }
    }
    pub fn to_set(&mut self) {
        self.mset.dedup();
    }
}

impl<T: Clone + Ord> Iterator for Permutations<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        // Lexicographic r-permutation generation
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
        let n = self.mset.len();
        let r = self.size;
        if n == 0 || r == 0 || r > n {
            (0, Some(0))
        } else {
            (1, Some(((n - r + 1)..=n).product()))
        }
    }
}

pub fn permutations<T: Clone + Ord>(s: &[T], size: usize) -> Permutations<T> {
    let mset = s.to_vec();
    Permutations::new(mset, size)
}

#[cfg(test)]
mod test {
    use crate::Permutations;
    #[test]
    fn tuple_size_check() {
        let raw_vec = vec![1, 3, 5, 7];
        let one = Permutations::new(raw_vec.clone(), 1);
        let two = Permutations::new(raw_vec.clone(), 2);
        let three = Permutations::new(raw_vec.clone(), 3);
        let four = Permutations::new(raw_vec.clone(), 4);
        assert_eq!(one.size, two.size - 1usize);
        assert_eq!(three.size_hint(), (1, Some(24)));
        assert_eq!(four.size_hint(), (1, Some(24)));
    }
}
