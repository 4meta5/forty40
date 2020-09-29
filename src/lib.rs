mod perm;
mod shift;
pub use perm::Permutation;
pub use shift::*;

/// Returns all permutations of the input string
pub fn full_shuffle(s: &str) -> Option<Vec<String>> {
    let s = s.as_bytes();
    if s.len() > 16usize {
        return None
    }
    let mut p = Permutation::new(s);
    Some(
        p.generate(s.len())
            .expect("max len is set len; qed")
            .into_iter()
            // TODO: replace unwrap with proper error propagation
            .map(|b| std::str::from_utf8(&b).unwrap().to_string())
            .collect::<Vec<String>>(),
    )
}
/// Fisher Yates Shuffle, pretty useless by itself right now, just shuffles strings
pub fn shuffle(s: &str) -> String {
    let len = s.to_string().chars().count();
    let shuffle = algo_p(len);
    let mut ret = String::new();
    shuffle.into_iter().for_each(|index| {
        if let Some(s) = s.to_string().chars().nth(index) {
            ret.push(s)
        }
    });
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_shuffle_works() {
        assert!(
            shuffle("Happy Birthday To You")
                != "Happy Birthday To You".to_string()
        );
    }
    #[test]
    fn full_shuffle_starts() {
        let s = "Happy";
        let mut p = Permutation::new(s.as_bytes());
        assert!(p.generate(s.as_bytes().len()).is_some());
    }
}
