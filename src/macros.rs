//! Shuffle vectors via index permutations

#[macro_export]
macro_rules! shuffle {
    ($vec:expr) => {
        match $vec {
            v => {
                let len = v.len();
                $crate::util::shift::shift(
                    &v,
                    $crate::util::shift::Transform::new(len).shuffle(),
                )
            }
        }
    };
}

#[macro_export]
macro_rules! permute {
    ($vec:expr) => {
        match $vec {
            v => {
                let len = v.len();
                use $crate::util::perm::GenerateR;
                $crate::util::perm::Permutation::new(v.as_slice()).generate(len)
            }
        }
    };
    ($vec:expr; $r:expr) => {
        match $vec {
            v => {
                use $crate::util::perm::GenerateR;
                $crate::util::perm::Permutation::new(v.as_slice()).generate($r)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::util::perm::GenerateR;
    #[test]
    fn check_shuffle_len_constant() {
        let s = vec![1, 2, 3, 4, 5, 6];
        let ss = s.clone();
        assert!(shuffle!(s).len() == ss.len());
    }
    #[test]
    fn check_r_permute() {
        let vec = vec![1, 2, 3, 4, 5];
        let p = crate::util::perm::Permutation::new(vec.as_slice());
        let n = p.generate(3);
        assert!(n == permute!(vec; 3));
    }
    #[test]
    fn more() {
        let cards = vec![1, 2, 3, 4]; // any T: Clone + Ord
        let cards_copy = cards.clone();
        let cc2 = cards.clone();
        let shuffled_cards = shuffle!(cards);
        assert!(shuffled_cards != cc2 && shuffled_cards.len() == cc2.len());
        let all_three_drawings = permute!(cards_copy; 3);
        assert!(all_three_drawings.is_some());
    }
}
