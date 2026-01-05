use std::collections::HashMap;

impl Solution {
    fn is_vowel(c: u8) -> bool {
        if c == b'a' || c == b'e' || c == b'i' || c == b'o' || c == b'u' {
            return true;
        }
        false
    }

    fn at_least(word: String, k: i32) -> i64 {
        let len = word.len();
        let word = word.as_bytes();
        let mut consonants = 0;
        let mut left = 0;
        let mut result = 0i64;
        let mut map = HashMap::<u8, i32>::new();
        for right in 0..len {
            if Self::is_vowel(word[right]) {
                map.entry(word[right]).and_modify(|x| *x += 1).or_insert(1);
            } else {
                consonants += 1;
            }

            while map.len() == 5_usize && consonants >= k {
                result += (len - right) as i64;
                if Self::is_vowel(word[left]) {
                    map.entry(word[left]).and_modify(|x| *x -= 1);
                    if map.get(&word[left]) == Some(&0) {
                        map.remove(&word[left]);
                    }
                } else {
                    consonants -= 1;
                }
                left += 1;
            }
        }

        result
    }

    pub fn count_of_substrings(word: String, k: i32) -> i64 {
        Self::at_least(word.clone(), k) - Self::at_least(word, k + 1)
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    // tracing::info!("{}", Solution::count_of_substrings("aeioqq".to_string(), 1));
    tracing::info!(
        "{}",
        Solution::count_of_substrings("ieaouqqieaouqq".to_string(), 1)
    );
}
