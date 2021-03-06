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
macro_rules! shuffle_str {
    ($str:expr) => {
        match $str {
            s => $crate::util::shift::string_shuffle(&s),
        }
    };
}

#[macro_export]
macro_rules! perm_next {
    ($vec:expr) => {
        match $vec {
            v => {
                use $crate::util::perm::PermIter;
                $crate::util::perm::Permutation(v).next()
            }
        }
    };
}

#[macro_export]
macro_rules! perm_last {
    ($vec:expr) => {
        match $vec {
            v => {
                use $crate::util::perm::PermIter;
                $crate::util::perm::Permutation(v).prev()
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
    fn check_shuffle_permute() {
        let cards = vec![1, 2, 3, 4];
        let cards_copy = cards.clone();
        let cc2 = cards.clone();
        let shuffled_cards = shuffle!(cards);
        assert!(shuffled_cards != cc2 && shuffled_cards.len() == cc2.len());
        let all_three_drawings = permute!(cards_copy; 3);
        assert!(all_three_drawings.is_some());
    }
    #[test]
    fn next_last() {
        let cards = vec![1, 2, 3, 4];
        assert!(perm_next!(cards.clone()).is_some());
        let one = perm_next!(cards.clone()).unwrap();
        assert!(perm_last!(one) == Some(cards));
    }
    #[test]
    fn shuffle_str_works() {
        let xa = "abcdefghijklmnopqrstuvwxyz";
        let xb = shuffle_str!(xa);
        for xc in xa.chars() {
            assert!(xb.contains(xc));
        }
        assert!(xb.len() == xa.len());
        let xd = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let xe = shuffle_str!(xd);
        for xf in xd.chars() {
            assert!(xe.contains(xf));
        }
        assert!(xd.len() == xe.len());
        let xg = "0123456789";
        let xh = shuffle_str!(xg);
        for xi in xg.chars() {
            assert!(xh.contains(xi));
        }
        assert!(xg.len() == xh.len());
        let xj = " !\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
        let xk = shuffle_str!(xj);
        for xl in xj.chars() {
            assert!(xk.contains(xl));
        }
        assert!(xj.len() == xk.len());
    }
}
