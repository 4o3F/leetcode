impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let chars = s.as_bytes();
        let (mut start_idx, mut end_idx) = ([-1; 26], [-1; 26]);
        for (idx, ch) in chars.iter().enumerate() {
            if start_idx[(*ch - b'a') as usize] == -1 {
                start_idx[(*ch - b'a') as usize] = idx as i32;
            }
            end_idx[(*ch - b'a') as usize] = idx as i32;
        }

        let mut result = 0;
        (0..26).for_each(|idx| {
            let mut appearance = [false; 26];
            if end_idx[idx] != -1 {
                for j in start_idx[idx] + 1..end_idx[idx] {
                    if !appearance[(chars[j as usize] - b'a') as usize] {
                        appearance[(chars[j as usize] - b'a') as usize] = true;
                        // tracing::debug!(
                        //     "{}{}{}",
                        //     (b'a' + idx as u8) as char,
                        //     chars[j as usize] as char,
                        //     (b'a' + idx as u8) as char
                        // );
                        result += 1;
                    }
                }
            }
        });

        result
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{}",
        Solution::count_palindromic_subsequence("bbcbaba".to_string())
    );
}
