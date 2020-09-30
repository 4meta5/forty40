mod mrep;
mod util;
use std::str;

/// Returns all permutations of the input string
pub fn full_shuffle(s: &str) -> Option<Vec<String>> {
    let s = s.as_bytes().to_vec();
    let len = s.len();
    if len > 16usize {
        return None
    }
    let p = permute!(s, len);
    let mut perms = Vec::<String>::new();
    for i in p.iter() {
        for j in i.iter() {
            if let Ok(text) = std::str::from_utf8(&j) {
                perms.push(text.to_string());
            }
        }
    }
    if perms.is_empty() {
        None
    } else {
        Some(perms)
    }
}
/// Fisher Yates Shuffle, pretty useless by itself right now, just shuffles strings
pub fn shuffle(s: &str) -> String {
    str::from_utf8(&permute!(s.as_bytes().to_vec()))
        .unwrap()
        .to_string()
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
