//! Plan is to benchmark shuffling implementations against each other and try to optimize tf out of them
//!
//! Lots of applications, including the shuffle-not-swap consensus primitive and block cipher constructions (see Feistel Networks)
use alloc::{
    collections::btree_map::BTreeMap,
    vec::{
        self,
        Vec,
    },
};
use rand::Rng;

#[derive(Clone)]
/// Permutation of Vec<T> via Fisher-Yates Shuffle
pub struct FisherYates<T> {
    vec: Vec<T>,
    state: Vec<usize>,
    counter: usize,
}

impl<T: Clone + Ord> FisherYates<T> {
    pub fn new(v: &[T]) -> Self {
        let vec = v.to_vec();
        let mut state = Vec::<usize>::new();
        // TODO: find a way to not require this loop, maybe generated from v
        for (i, _) in v.iter().enumerate() {
            state.push(i);
        }
        let counter = state.len(); // == vec.len() by construction
        FisherYates {
            vec,
            state,
            counter,
        }
    }
}

/// Fisher-Yates Shuffle, random in-place permutations with constant storage requirements
impl<T: Clone + Ord> Iterator for FisherYates<T> {
    type Item = Vec<usize>;
    fn next(&mut self) -> Option<Vec<usize>> {
        // TODO: only generate rng once instead of once per loop
        let mut rng = rand::thread_rng();
        if self.counter > 0usize {
            self.counter -= 1usize;
            let j = rng.gen_range(0, self.counter + 1usize);
            let temp = self.state[self.counter];
            if j != self.counter {
                self.state[self.counter] = self.state[j];
            }
            self.state[j] = temp;
            Some(self.state.clone())
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.vec.len();
        if n == 0 {
            (0, Some(0))
        } else {
            (1, Some((1..n + 1).product())) // n!
        }
    }
}

#[derive(Clone)]
/// Permutation of Vec<T> via Sattolo Cycles
pub struct SatCycles<T> {
    vec: Vec<T>,
    state: Vec<usize>,
    counter: usize,
}

impl<T: Clone + Ord> SatCycles<T> {
    pub fn new(v: &[T]) -> Self {
        let vec = v.to_vec();
        let mut state = Vec::<usize>::new();
        // TODO: find a way to not require this loop, maybe generated from v
        for (i, _) in v.iter().enumerate() {
            state.push(i);
        }
        let counter = state.len(); // == vec.len() by construction
        SatCycles {
            vec,
            state,
            counter,
        }
    }
}

/// Sattolo Cycles, random in-place permutations with constant storage requirements
impl<T: Clone + Ord> Iterator for SatCycles<T> {
    type Item = Vec<usize>;
    fn next(&mut self) -> Option<Vec<usize>> {
        // TODO: only generate rng once instead of once per loop
        let mut rng = rand::thread_rng();
        if self.counter > 1usize {
            self.counter -= 1usize;
            let j = rng.gen_range(0, self.counter);
            let temp = self.state[self.counter];
            self.state[self.counter] = self.state[j];
            self.state[j] = temp;
            Some(self.state.clone())
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.vec.len();
        if n == 0 {
            (0, Some(0))
        } else {
            (1, Some((1..n + 1).product())) // n!
        }
    }
}

pub trait Transform {
    fn transform(&mut self) -> bool;
}
/// Transforms the Vec using the state of the index permutation
/// -> Returns true if transformed
impl<T: Clone> Transform for SatCycles<T> {
    fn transform(&mut self) -> bool {
        // TODO: make two inner loops one loop
        if self.state.len() == self.vec.len() {
            let mut temp: BTreeMap<usize, T> = BTreeMap::new();
            let mut ret_early = false;
            self.state.iter().for_each(|index| {
                if index >= &self.vec.len() {
                    // (i) => one of the indices is equal to or exceeds the length of the vector itself
                    ret_early = true;
                } else {
                    if temp.get(index).is_none() {
                        temp.insert(
                            *index,
                            self.vec
                                .get(*index)
                                .expect("checked len and index < len; qed")
                                .clone(),
                        );
                    } else {
                        // (ii) => duplicates in input vec
                        ret_early = true;
                    }
                }
            });
            if ret_early {
                // if (i) or (ii) are true, we cannot perform the transform because the input is unsafe
                return false
            }
            // remap elements from current(0, 1, 2, ...) to passed in indices
            let mut new_inner = Vec::<T>::new();
            for j in 0..self.state.len() {
                let item = temp
                    .get(&j)
                    .expect(
                        "checked for duplicates and length so should fully cover index range; qed",
                    )
                    .clone();
                new_inner.push(item);
            }
            self.vec = new_inner;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;
    #[test]
    fn size_hints_work() {
        let ex = vec![5, 7, 19, 21, 36];
        let mut f = FisherYates::new(ex.as_slice());
        assert_eq!(f.size_hint(), (1, Some(120)));
        assert!(f.next().is_some());
        let mut s = SatCycles::new(ex.as_slice());
        assert_eq!(s.size_hint(), (1, Some(120)));
        assert!(s.next().is_some());
    }
}
