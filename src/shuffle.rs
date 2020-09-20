use alloc::{collections::btree_map::BTreeMap, vec::Vec};
use rand::Rng;

#[derive(Clone)]
/// Permutation of Vec<T>
pub struct Perm<T> {
    vec: Vec<T>,
    state: Vec<usize>,
    counter: usize,
}

impl<T: Clone + Ord> Perm<T> {
    pub fn new(v: &[T]) -> Self {
        let vec = v.to_vec();
        let mut state = Vec::<usize>::new();
        // TODO: find a way to not require this loop, maybe generated from v
        for (i, _) in v.iter().enumerate() {
            state.push(i);
        }
        let counter = state.len(); // == vec.len() by construction
        Perm {
            vec,
            state,
            counter: counter,
        }
    }
}

/// Sattolo Cycles, random in-place permutations with constant storage requirements
impl<T: Clone + Ord> Iterator for Perm<T> {
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
}

pub trait Transform {
    fn transform(&mut self) -> bool;
}
/// Transforms the Vec using the state of the index permutation
impl<T: Clone> Transform for Perm<T> {
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
                return false;
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
