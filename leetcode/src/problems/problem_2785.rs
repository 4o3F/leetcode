impl Solution {
    pub fn sort_vowels(s: String) -> String {
        let vowels: Vec<char> = "aeiouAEIOU".chars().collect();

        let is_vowel = |c: char| vowels.contains(&c);

        let mut extracted: Vec<char> = s.chars().filter(|&c| is_vowel(c)).collect();

        extracted.sort_by(|a, b| (*a as u8).cmp(&(*b as u8)));

        let mut result = String::new();
        let mut idx = 0;

        for c in s.chars() {
            if is_vowel(c) {
                result.push(extracted[idx]);
                idx += 1;
            } else {
                result.push(c);
            }
        }

        result
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::sort_vowels("lEetcOde".to_string()))
}
