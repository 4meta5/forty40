use std::collections::HashSet;

pub struct Transform(pub Vec<usize>);

impl Transform {
    fn new(len: usize) -> Self {
        Transform((0..len).map(|a| a).collect::<Vec<usize>>())
    }
    fn is_valid(&self) -> bool {
        let h = HashSet::with_capacity(self.0.len());
        let mut counter = 0usize;
        for i in self.0.iter() {
            if !h.insert(i) {
                return false; // duplicate elements not allowed
            } else {
                counter += 1usize;
            }
        }
        // every element must be unique as well
        if counter != self.0.len() {
            return false;
        }
        true
    }
}

/// Apply an index permutation to the input vector v
pub fn shift<T>(v: Vec<T>, t: Transform) -> Option<Vec<T>> {
    if v.len() != t.0.len() || !t.is_valid() {
        return None
    } else {
        let mut ret = Vec::<T>::new();
        todo!()
    }
}