//! Shuffle vectors via index permutations. There are two match arms for `permute!`
//! 1. (vec) -> Fisher-Yates to shuffle the indices of the vector and apply the index transformation to the original vector
//! 2. (vec, usize) -> Lexicographic r-permutation generates where r is the size of the input usize

#[macro_export]
macro_rules! permute {
    ($vec:expr) => {
        match $vec {
            v => {
                let len = v.len();
                crate::util::shift::shift(
                    &v,
                    crate::util::shift::Transform::new(len).shuffle(),
                )
            }
        }
    };
    ($vec:expr, $r:expr) => {
        match $vec {
            v => crate::util::perm::Permutation::new(v.as_slice()).generate($r),
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_shuffle_len_constant() {
        let s = "beeboopbopboop";
        let ss = s.as_bytes().to_vec();
        assert!(permute!(ss.clone()).len() == ss.len());
    }
    #[test]
    fn check_r_permute() {
        let vec = vec![1, 2, 3, 4, 5];
        let mut p = crate::util::perm::Permutation::new(vec.as_slice());
        let n = p.generate(3);
        assert!(n == permute!(vec, 3));
    }
    #[test]
    fn more() {
        let cards = vec![1, 2, 3, 4]; // any T: Clone + Ord
        let cards_copy = cards.clone();
        let cc2 = cards.clone();
        let shuffled_cards = permute!(cards);
        assert!(shuffled_cards != cc2 && shuffled_cards.len() == cc2.len());
        let all_three_drawings = permute!(cards_copy, 3);
        let expected_all_three =
            vec![vec![1, 2, 3], vec![1, 2, 4], vec![1, 3, 4], vec![2, 3, 4]];
        assert!(expected_all_three.len() == all_three_drawings.unwrap().len());
    }
}
