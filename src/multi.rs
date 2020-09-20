//! Multipermutations and combinations
use alloc::vec::{
    self,
    Vec,
};

#[derive(Clone)]
pub struct JohnsonTrotter<T> {
    vec: Vec<T>,
    subsize: usize,
    pointer: usize,
    counter: [u8; MAXHEAP - 1],
}

impl<T: Clone + Ord> JohnsonTrotter<T> {
    pub fn new(v: &[T], subsize: usize) -> JohnsonTrotter<T> {
        let vec = v.to_vec();
        JohnsonTrotter {
            vec,
            subsize,
            pointer: 0usize,
            counter: [0u8; MAXHEAP - 1],
        }
    }
}

/// Steinhaus-Johnson-Trotter algorithm for generating in-place permutations
/// -> does not require the `Transform` call after permutation (TODO: bench differences)
impl<T: Clone + Ord> Iterator for JohnsonTrotter<T> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Vec<T>> {
        todo!()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (n, r) = (self.vec.len(), self.subsize);
        if n == 0 || r == 0 || r > n {
            (0, Some(0))
        } else {
            (1, Some(((n - r + 1)..n + 1).product())) // n! / (n - r)!
        }
    }
}

/// Maximum number of elements we can generate permutations for using
/// Heap's algorithm
pub const MAXHEAP: usize = 16;

#[derive(Clone)]
pub struct Heaps<T> {
    vec: Vec<T>,
    subsize: usize,
    pointer: usize,
    counter: [u8; MAXHEAP - 1],
}

impl<T: Clone + Ord> Heaps<T> {
    pub fn new(v: &[T], subsize: usize) -> Heaps<T> {
        let vec = v.to_vec();
        Heaps {
            vec,
            subsize,
            pointer: 0usize,
            counter: [0u8; MAXHEAP - 1],
        }
    }
}

/// Heaps algorithm for generating in-place permutations
/// -> does not require the `Transform` call after permutation (TODO: bench differences)
impl<T: Clone + Ord> Iterator for Heaps<T> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Vec<T>> {
        // if self.counter < self.vec.len() {
        //     if self.stack[self.pointer] < self.pointer {
        //         if self.pointer % 2 == 0 {
        //             // swap(vec[0], vec[i])
        //             self.vec.swap(0usize, self.pointer);
        //         } else {
        //             // swap(vec[stack[i]], vec[i])
        //             self.vec.swap(self.stack[self.pointer], self.pointer);
        //         }
        //         Some(self.vec.clone())
        //     } else {
        //         // pop the stack
        //         self.stack[self.pointer] = 0usize;
        //         self.pointer += 1usize;
        //         None
        //     }
        // } else {
        //     None
        // }
        todo!()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (n, r) = (self.vec.len(), self.subsize);
        if n == 0 || r == 0 || r > n {
            (0, Some(0))
        } else {
            (1, Some(((n - r + 1)..n + 1).product())) // n! / (n - r)!
        }
    }
}
