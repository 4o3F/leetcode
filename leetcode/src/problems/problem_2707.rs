use std::{cmp::Reverse, collections::BinaryHeap};

// TODO: Reimplement this
impl Solution {
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        let mut dict: Vec<Vec<String>> = vec![Vec::new(); 26];
        for word in dictionary {
            let word_idx = (word.as_bytes()[0] - b'a') as usize;
            dict[word_idx].push(word);
        }

        let ss = s.as_str();
        let sb = s.as_bytes();
        let mut heap: BinaryHeap<(Reverse<i32>, usize)> = BinaryHeap::new();
        heap.push((Reverse(0), 0));
        while let Some((Reverse(cost), next)) = heap.pop() {
            if next == s.len() {
                return cost;
            }

            let word_idx = (sb[next] - b'a') as usize;
            for word in &dict[word_idx] {
                if next + word.len() <= s.len() && word.eq(&ss[next..next + word.len()]) {
                    heap.push((Reverse(cost), next + word.len()));
                }
            }

            heap.push((Reverse(cost + 1), next + 1));
        }

        s.len() as i32
    }
}
struct Solution {}
pub fn run() {
    tracing::info!(
        "{}",
        Solution::min_extra_char(
            "leetscode".to_string(),
            vec!["leet".to_string(), "code".to_string(), "leetcode".to_string()]
        )
    )
}
