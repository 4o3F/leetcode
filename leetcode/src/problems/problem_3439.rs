impl Solution {
    pub fn max_free_time(event_time: i32, k: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        let mut blocks = Vec::new();
        blocks.push(start_time[0]);
        for i in 1..start_time.len() {
            blocks.push(start_time[i] - end_time[i - 1]);
        }
        blocks.push(event_time - end_time[end_time.len() - 1]);
        // sliding window
        let mut left = 0;
        let mut right = 0;
        let mut ans = 0;
        let mut sum = 0;
        while right < blocks.len() {
            if right - left > k as usize {
                sum -= blocks[left];
                left += 1;
            } else {
                sum += blocks[right];
                right += 1;
            }
            ans = ans.max(sum);
        }
        ans
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::max_free_time(5, 1, vec![1, 3], vec![2, 5]))
}
