//! Apply Permutations to Vec<T> by a permutation of the index vector
use rand::Rng;

pub struct Transform(pub Vec<usize>);

impl Transform {
    pub fn new(len: usize) -> Self {
        Transform((0..len).collect::<Vec<usize>>())
    }
    pub fn shuffle(&mut self) -> Self {
        Transform(fy_shuffle(self.0.len()))
    }
    pub fn is_valid(&self) -> bool {
        for i in self.0.iter() {
            if i > &self.0.len() {
                return false
            }
            let mut c = 0usize;
            for j in self.0.iter() {
                if i == j {
                    c += 1usize;
                }
                // any duplicates
                if c > 1 {
                    return false
                }
            }
        }
        true
    }
}

/// Apply an index permutation to the input vector v
pub fn shift<T: Clone>(v: &[T], t: Transform) -> Vec<T> {
    if v.len() != t.0.len() || !t.is_valid() {
        panic!()
    }
    let mut ret = Vec::<T>::new();
    t.0.into_iter().for_each(|a| {
        ret.push(v[a].clone());
    });
    ret
}

/// [src]: https://en.wikipedia.org/wiki/Fisher–Yates_shuffle#The_modern_algorithm
pub fn fy_shuffle(len: usize) -> Vec<usize> {
    let mut state = (0..len).collect::<Vec<usize>>();
    let mut rng = rand::thread_rng();
    let mut counter = state.len() - 1usize;
    while counter > 0 {
        let j = rng.gen_range(0, counter);
        if counter != j {
            state.swap(counter, j);
        }
        counter -= 1usize;
    }
    state
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str;
    #[test]
    fn transform_meets_validity_defn() {
        let s = vec![1, 2, 3, 4, 5, 6];
        let t = fy_shuffle(s.len());
        let tr = Transform(t);
        assert!(tr.is_valid());
    }
    #[test]
    fn transform_works() {
        let s = vec![1, 2, 3, 4, 5, 6];
        let t = fy_shuffle(s.len());
        let tr = Transform(t.clone());
        let mut ret = Vec::new();
        // manual transform applied to string
        t.into_iter().for_each(|index| ret.push(s[index]));
        let st = shift(&s, tr);
        assert!(ret == st);
    }
    #[test]
    fn shuffle_is_valid() {
        let s = vec![1, 2, 3, 4, 5, 6];
        let mut tr = Transform::new(s.len());
        let sh = tr.shuffle();
        assert!(sh.is_valid());
    }
}
