impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        arr.into_iter()
            .enumerate()
            .fold((0, 0), |(mut sum, mut result), (index, num)| {
                sum += num;
                if sum == (index * (index + 1) / 2) as i32 {
                    result += 1;
                }
                (sum, result)
            })
            .1
    }
}

struct Solution {}
pub fn run() {
    tracing::info!("{}", Solution::max_chunks_to_sorted(vec![4, 3, 2, 1, 0]));
}
