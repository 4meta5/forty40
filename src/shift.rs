//! Apply Permutations to Vec<T> by a permutation of the index vector
use rand::Rng;

pub struct Transform(pub Vec<usize>);

impl Transform {
    pub fn new(len: usize) -> Self {
        Transform((0..len).map(|a| a).collect::<Vec<usize>>())
    }
    pub fn is_valid(&self) -> bool {
        for i in self.0.iter() {
            if i > &self.0.len() {
                return false
            }
            for j in self.0.iter() {
                // any duplicates
                if i == j {
                    return false
                }
            }
        }
        true
    }
}

/// Apply an index permutation to the input vector v
pub fn shift<T: Clone>(v: Vec<T>, t: Transform) -> Option<Vec<T>> {
    if v.len() != t.0.len() || !t.is_valid() {
        return None
    } else {
        let mut ret = Vec::<T>::new();
        t.0.into_iter().for_each(|a| {
            ret.push(v[a].clone());
        });
        Some(ret)
    }
}

/// [src]: https://en.wikipedia.org/wiki/Fisher–Yates_shuffle#The_modern_algorithm
pub fn algo_p(len: usize) -> Vec<usize> {
    let mut state = Vec::<usize>::new();
    for i in 0..len {
        state.push(i);
    }
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
    fn transform_does_not_effect_len() {
        let s = "bee boop bop boop";
        let t = algo_p(s.len());
        let tr = Transform(t.clone());
        assert!(tr.is_valid());
        let mut ret = String::new();
        t.into_iter().for_each(|index| {
            if let Some(x) = s.to_string().chars().nth(index) {
                ret.push(x)
            }
        });
        let st: String =
            str::from_utf8(&shift(s.as_bytes().to_vec(), tr).unwrap())
                .unwrap()
                .to_string();
        assert!(ret == st);
    }
}
