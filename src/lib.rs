mod macros;
mod util;
use std::str;

/// Returns all permutations of the input string
pub fn full_shuffle(s: &str) -> Option<Vec<String>> {
    let len = s.len();
    if len > 16usize {
        return None
    }
    permute_str!(s.to_string())
}
/// Fisher Yates Shuffle, pretty useless by itself right now, just shuffles strings
pub fn shuffle(s: &str) -> String {
    shuffle_str!(s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_shuffle_works() {
        assert!(shuffle("Happy Birthday To You") != "Happy Birthday To You");
    }
    #[test]
    fn full_shuffle_starts() {
        let s = "Happy";
        assert!(full_shuffle(s).is_some());
    }
}
