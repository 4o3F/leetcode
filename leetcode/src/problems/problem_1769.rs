impl Solution {
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let prefix = boxes.bytes().fold((vec![], 0), |(mut prefix, count), c| {
            prefix.push(prefix.last().unwrap_or(&0) + count);
            (prefix, count + if c == b'1' { 1 } else { 0 })
        }).0;
        let mut suffix = boxes.bytes().rev().fold((vec![], 0), |(mut suffix, count), c| {
            suffix.push(suffix.last().unwrap_or(&0) + count);
            (suffix, count + if c == b'1' { 1 } else { 0 })
        }).0;

        suffix.reverse();

        let result = prefix
            .iter()
            .zip(suffix.iter())
            .map(|(a, b)| a + b)
            .collect::<Vec<i32>>();

        // tracing::debug!("{:?}", prefix);
        // tracing::debug!("{:?}", suffix);
        result
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{:?}", Solution::min_operations("110".to_string()));
    tracing::info!("{:?}", Solution::min_operations("001011".to_string()));
}
