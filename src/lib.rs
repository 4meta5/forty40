mod heap;
mod lex;
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

/// Factorial function for calculating n! for some n
fn fac(n: usize) -> usize {
    (1..n + 1).fold(1, |a, b| a * b)
}

/// Permutation
/// if repetition { n choices every time, r times => n^r }
/// else { n!/(n-r)! }
pub fn perm(subset_size: usize, set_size: usize, repetition: bool) -> usize {
    if repetition {
        return set_size.pow(subset_size as u32);
    } else {
        let (num, dom) = (fac(set_size), fac(set_size - subset_size));
        num / dom
    }
}

/// Combination, order doesn't matter
/// if repetition { (r + n - 1)! / r!(n-1)! }
/// else { n!/r!(n-r)! }
pub fn comb(subset_size: usize, set_size: usize, repetition: bool) -> usize {
    if repetition {
        let (num, mut dom) = (fac(subset_size + set_size - 1), fac(set_size - 1));
        dom *= fac(subset_size);
        num / dom
    } else {
        let (num, mut dom) = (fac(set_size), fac(set_size - subset_size));
        dom *= fac(subset_size);
        num / dom
    }
}

/// Theorem 2.4.1
/// Let S be a multiset with `k` different types with infinite repetition. Then
/// the number of r-permutations of S is `k^{r}`
pub fn r_perms<T: PartialEq + Clone>(r: usize, v: &[T]) -> usize {
    let mut vec = v.to_vec();
    vec.dedup();
    perm(r, vec.len(), true)
}

// Theorem 2.4.2 - Permutate Multiset
// Let S be a multiset with objects of `k` different types with finite representation
// numbers `n_{1}, n_{2}, n_{3}, ..., n_{k}` respectively. |S| = \sum_{i=1..k}{n_{i}}
// The number of permutations of S equals n! / (n_{1}! * n_{2}! * ... * n_{k}! )
// i.e. permutations of MISSISSIPPI = 11! / (4!4!2!)
pub fn m_perms<T: PartialEq + Clone + Hash + Eq>(v: &[T]) -> usize {
    let vec = v.to_vec();
    let num = fac(vec.len());
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
        .for_each(|f| dom *= fac(f));
    num / dom
}

/*
Theorem 2.4.3
Let n be a positive integer and let n_{1}, .., n_{k} be positive integers s.t. n = n_{1} + ... + n_{k}. The number of ways to partition a set of n objects into k labeled boxes in which Box 1 contains n_{1} objects, Box 2 contains n_{2} objects, ..., Box k contains n_{k} objects equals n! / (n_{1}! * n_{2}! * ... * n_{k}!)

If the boxes are not labeled and n_{1} = ... = n_{k}, then the number of partitions equals n! / (k!n_{1}!n_{2}! * ... * n_{k}!)
*/
pub fn mbox_perms<T: PartialEq + Clone + Hash + Eq>(
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
        assert_eq!(perm(2, 5, true), 25);
        assert_eq!(perm(2, 2, true), 4);
        assert_eq!(perm(2, 5, false), 20);
    }
    #[test]
    fn comb_tests() {
        assert_eq!(comb(2, 5, true), 15);
        assert_eq!(comb(2, 2, true), 3);
        assert_eq!(comb(2, 5, false), 10);
    }
    #[test]
    fn r_perms_tests() {
        let vec = vec![1, 3, 5, 7, 9, 9, 11];
        assert_eq!(r_perms(2, vec.as_slice()), 36);
        assert_eq!(r_perms(3, vec.as_slice()), 216);
        assert_eq!(r_perms(4, vec.as_slice()), 1296);
    }
    #[test]
    fn m_perms_tests() {
        let vec = vec![1, 3, 5, 7, 9, 9, 11];
        // 7! / 2! = 5040 / 2 = 2520
        assert_eq!(m_perms(vec.as_slice()), 2520);
        let vec = vec![1, 3, 9, 9, 9, 9, 11];
        // 7! / 4! = 210
        assert_eq!(m_perms(vec.as_slice()), 210);
    }
    #[test]
    fn mbox_perms_tests() {
        let things = vec![1, 3, 4, 5, 7, 10];
        let boxes = vec![2usize, 3usize];
        //  6!/2!3! = 60
        assert_eq!(mbox_perms(things.as_slice(), boxes.as_slice(), true), 60);
        // 60 / k! == 2! => 30
        assert_eq!(mbox_perms(things.as_slice(), boxes.as_slice(), false), 30);
    }
}
