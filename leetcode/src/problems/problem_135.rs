impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut result = vec![1; ratings.len()];
        let mut idx = 0;
        while idx < ratings.len() - 1 {
            if ratings[idx] < ratings[idx + 1] {
                result[idx + 1] = result[idx] + 1;
            }
            idx += 1;
        }
        while idx > 0 {
            if ratings[idx] < ratings[idx - 1] {
                result[idx - 1] = result[idx - 1].max(result[idx] + 1);
            }
            idx -= 1;
        }
        tracing::info!("{:?}", result);
        result.iter().sum()
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::candy(vec![1, 3, 4, 5, 2]));
}
