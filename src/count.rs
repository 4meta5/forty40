//! ## Counting Rules
use std::{collections::HashMap, hash::Hash};

#[derive(Clone)]
pub struct Elem<T> {
    freq: usize,
    item: T,
}
impl<T: Clone> Elem<T> {
    pub fn new(freq: usize, item: T) -> Self {
        Self { freq, item }
    }
    pub fn freq(&self) -> usize {
        self.freq
    }
}
// wrapper required to circumvent inability to impl From<Vec<T>>
pub struct Vector<T>(pub Vec<T>);
// should be de-duplicated by construction, see From<Vector<T>> impl below
pub type MultiSet<T> = Vec<Elem<T>>;

impl<T: Clone + Eq + Hash> From<Vector<T>> for MultiSet<T> {
    fn from(other: Vector<T>) -> MultiSet<T> {
        let mut frequencies: HashMap<T, usize> = HashMap::new();
        other.0.into_iter().for_each(|n| {
            if let Some(v) = frequencies.get(&n) {
                frequencies.insert(n, v + 1usize);
            } else {
                frequencies.insert(n, 1usize);
            }
        });
        let mut ret = MultiSet::<T>::new();
        for (key, value) in frequencies.iter() {
            let e = Elem::new(*value, key.clone());
            ret.push(e);
        }
        ret
    }
}

fn factorial(n: usize) -> usize {
    fn helper(acc: usize, m: usize) -> usize {
        match m {
            0 => acc,
            _ => helper(acc * m, m - 1),
        }
    }
    helper(1, n)
}

/// Theorem 2.4.1
/// Let S be a multiset with `k` different types with infinite repetition. Then
/// the number of r-permutations of S is `k^{r}`
pub fn num_r_perm_inf_rep<T: PartialEq + Clone>(r: u32, v: &[T]) -> usize {
    let mut vec = v.to_vec();
    vec.dedup();
    vec.len().pow(r)
}

// Theorem 2.4.2 - Permutate Multiset
// Let S be a multiset with objects of `k` different types with finite representation
// numbers `n_{1}, n_{2}, n_{3}, ..., n_{k}` respectively. |S| = \sum_{i=1..k}{n_{i}}
// The number of permutations of S equals n! / (n_{1}! * n_{2}! * ... * n_{k}! )
// i.e. permutations of MISSISSIPPI = 11! / (4!4!2!)
pub fn num_permute_mset<T: PartialEq + Clone + Hash + Eq>(v: &[T]) -> usize {
    let vec = v.to_vec();
    let num = factorial(vec.len());
    fn get_unique_frequencies<S: Eq + Hash>(vector: Vec<S>) -> Vec<usize> {
        let mut frequencies: HashMap<S, usize> = HashMap::new();
        vector.into_iter().for_each(|n| {
            if let Some(v) = frequencies.get(&n) {
                frequencies.insert(n, v + 1usize);
            } else {
                frequencies.insert(n, 1usize);
            }
        });
        let mut ret = Vec::<usize>::new();
        for val in frequencies.values() {
            ret.push(*val)
        }
        ret
    }
    let mut dom = 1usize;
    get_unique_frequencies::<T>(vec)
        .into_iter()
        .for_each(|f| dom *= factorial(f));
    num / dom
}

/*
Theorem 2.4.3
Let n be a positive integer and let n_{1}, .., n_{k} be positive integers s.t. n = n_{1} + ... + n_{k}. The number of ways to partition a set of n objects into k labeled boxes in which Box 1 contains n_{1} objects, Box 2 contains n_{2} objects, ..., Box k contains n_{k} objects equals n! / (n_{1}! * n_{2}! * ... * n_{k}!)

If the boxes are not labeled and n_{1} = ... = n_{k}, then the number of partitions equals n! / (k!n_{1}!n_{2}! * ... * n_{k}!)
*/
pub fn num_into_boxes<T: PartialEq + Clone + Hash + Eq>(
    v: &[T],
    boxes: &[usize],
    labeled: bool,
) -> usize {
    let (vec, boxes) = (v.to_vec(), boxes.to_vec());
    let num = factorial(vec.len());
    let mut dom = 1usize;
    // number of boxes
    let k = boxes.len();
    boxes.into_iter().for_each(|space| dom *= factorial(space));
    if !labeled {
        dom *= factorial(k);
    }
    num / dom
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn fac() {
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(6), 720);
        assert_eq!(factorial(7), 5040);
        assert_eq!(factorial(8), 40320);
        assert_eq!(factorial(9), 362880);
        assert_eq!(factorial(10), 3628800);
    }
}
