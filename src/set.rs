//! [src](https://github.com/open-web3-stack/open-runtime-module-library/blob/master/utilities/src/ordered_set.rs)

/// An multiset backed by `Vec`
#[derive(PartialEq, Eq, Default, Debug)]
pub struct MultiSet<T>(pub Vec<T>);

impl<T: Ord> MultiSet<T> {
    /// Create a new empty set
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Create a set from a `Vec`.
    /// `v` will be sorted first.
    pub fn from(mut v: Vec<T>) -> Self {
        v.sort();
        Self::from_sorted_set(v)
    }

    /// Create a set from a `Vec`.
    /// Assume `v` is sorted.
    pub fn from_sorted_set(v: Vec<T>) -> Self {
        Self(v)
    }

    /// Clear the set
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Tail is the string containing its elements in non-increasing order
    pub fn tail(self) -> Vec<T> {
        self.0.into_iter().rev().map(|a| a).collect::<Vec<T>>()
    }
}

impl<T: Ord> From<Vec<T>> for MultiSet<T> {
    fn from(v: Vec<T>) -> Self {
        Self::from(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from() {
        let v = vec![4, 2, 3, 4, 3, 1];
        let set: MultiSet<i32> = v.into();
        assert_eq!(set, MultiSet::from(vec![1, 2, 3, 3, 4, 4]));
    }

    #[test]
    fn clear() {
        let mut set: MultiSet<i32> = MultiSet::from(vec![1, 2, 3, 4]);
        set.clear();
        assert_eq!(set, MultiSet::new());
    }
}
