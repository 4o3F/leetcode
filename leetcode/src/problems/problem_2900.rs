impl Solution {
    pub fn get_longest_subsequence(mut words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        result.push(std::mem::take(&mut words[0]));
        for idx in 0..groups.len() - 1 {
            if groups[idx + 1] != groups[idx] {
                result.push(std::mem::take(&mut words[(idx + 1) as usize]));
            }
        }

        result
    }
}

struct Solution;

pub fn run() {
    let words = vec!["a", "b", "c", "d"]
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>();
    let groups = vec![1, 0, 1, 1];
    tracing::info!("{:?}", Solution::get_longest_subsequence(words, groups))
}
