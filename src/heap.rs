//! Heap's algorithm for generating permutations
use std::hash::Hash;
/// 16! is hard upper bound on number of permutations generated at a time
pub const MAXHEAP: usize = 16;

#[derive(Clone)]
pub struct Heap<T> {
    state: Vec<T>,
    n: u32,
    // counter[i] is for the (i + 1) th element
    counter: [u8; MAXHEAP - 1],
}

impl<T: PartialEq + Clone + Hash + Eq> Heap<T> {
    pub fn new(s: &[T]) -> Option<Self> {
        let state = s.to_vec();
        if state.len() > MAXHEAP {
            None
        } else {
            //sort or deduplicate vec??
            Some(Heap {
                state,
                n: 0,
                counter: [0; MAXHEAP - 1],
            })
        }
    }
    /// Resets the counter, without changing the state
    pub fn reset_counter(&mut self) {
        self.n = !0;
        for c in &mut self.counter[..] {
            *c = 0;
        }
    }
}

impl<T: PartialEq + Clone + Hash + Eq + Ord> Iterator for Heap<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        while 1 + (self.n as usize) < self.state.len() {
            let (n, nu) = (self.n as u8, self.n as usize);
            // `counter[n]` is `i` in recursion
            if self.counter[nu] <= n {
                // `n, nu` track current length - 2
                let j = if nu % 2 == 0 {
                    self.counter[nu] as usize
                } else {
                    0
                };
                self.state.swap(j, nu + 1);
                self.counter[nu] += 1;
                self.n = 0;
                return Some(self.state.clone());
            } else {
                self.counter[nu] = 0;
                self.n += 1;
            }
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.state.len() == 0 || self.n as usize > self.state.len() {
            (0, Some(0))
        } else {
            (1, Some(crate::m_perms(self.state.as_slice())))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::iter::Iterator;
    #[test]
    fn test_heap() {
        let raw_vec: Vec<u32> = vec![1, 3, 5, 7];
        let mut h = Heap::new(raw_vec.as_slice()).unwrap();
        assert_eq!(h.size_hint(), (1, Some(24)));
        assert_eq!(h.next(), Some(vec![3, 1, 5, 7]));
        assert_eq!(h.next(), Some(vec![5, 1, 3, 7]));
        assert_eq!(h.next(), Some(vec![1, 5, 3, 7]));
        assert_eq!(h.next(), Some(vec![3, 5, 1, 7]));
        assert_eq!(h.next(), Some(vec![5, 3, 1, 7]));
        assert_eq!(h.next(), Some(vec![7, 3, 1, 5]));
        assert_eq!(h.next(), Some(vec![3, 7, 1, 5]));
        assert_eq!(h.next(), Some(vec![1, 7, 3, 5]));
        assert_eq!(h.next(), Some(vec![7, 1, 3, 5]));
        assert_eq!(h.next(), Some(vec![3, 1, 7, 5]));
        assert_eq!(h.next(), Some(vec![1, 3, 7, 5]));
        assert_eq!(h.next(), Some(vec![1, 5, 7, 3]));
        assert_eq!(h.next(), Some(vec![5, 1, 7, 3]));
        assert_eq!(h.next(), Some(vec![7, 1, 5, 3]));
        assert_eq!(h.next(), Some(vec![1, 7, 5, 3]));
        assert_eq!(h.next(), Some(vec![5, 7, 1, 3]));
        assert_eq!(h.next(), Some(vec![7, 5, 1, 3]));
        assert_eq!(h.next(), Some(vec![7, 5, 3, 1]));
        assert_eq!(h.next(), Some(vec![5, 7, 3, 1]));
        assert_eq!(h.next(), Some(vec![3, 7, 5, 1]));
        assert_eq!(h.next(), Some(vec![7, 3, 5, 1]));
        assert_eq!(h.next(), Some(vec![5, 3, 7, 1]));
        assert_eq!(h.next(), Some(vec![3, 5, 7, 1]));
        assert_eq!(h.next(), None);
    }
}
