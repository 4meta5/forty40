//! Shuffle vectors and strings via index permutations

pub(crate) fn bits_to_string(b: &[u8]) -> String {
    std::str::from_utf8(b).unwrap().to_string()
}
pub(crate) fn bytes_to_strings(b: Vec<Vec<u8>>) -> Vec<String> {
    let mut ret = Vec::<String>::new();
    for a in b.iter() {
        ret.push(std::str::from_utf8(a).unwrap().to_string());
    }
    ret
}

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
            s => {
                let len = s.len();
                $crate::macros::bits_to_string(
                    $crate::util::shift::shift(
                        &s.as_bytes().to_vec(),
                        $crate::util::shift::Transform::new(len).shuffle(),
                    )
                    .as_slice(),
                )
            }
        }
    };
}

#[macro_export]
macro_rules! permute_str {
    ($str:expr) => {
        match $str {
            s => {
                let len = s.len();
                if let Some(p) = $crate::util::perm::from_str(s).generate(len) {
                    Some($crate::macros::bytes_to_strings(p))
                } else {
                    None
                }
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
                $crate::util::perm::Permutation::new(v.as_slice()).generate(len)
            }
        }
    };
    ($vec:expr; $r:expr) => {
        match $vec {
            v => {
                $crate::util::perm::Permutation::new(v.as_slice()).generate($r)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_shuffle_len_constant() {
        let s = "beeboopbopboop";
        let ss = s.as_bytes().to_vec();
        assert!(shuffle!(ss.clone()).len() == ss.len());
    }
    #[test]
    fn check_r_permute() {
        let vec = vec![1, 2, 3, 4, 5];
        let mut p = crate::util::perm::Permutation::new(vec.as_slice());
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
