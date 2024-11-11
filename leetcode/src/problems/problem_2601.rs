impl Solution {
    pub fn prime_sub_operation(mut nums: Vec<i32>) -> bool {
        let primes = (2..*nums.iter().max().unwrap())
            .filter(|&i| (2..i).all(|j| i % j != 0))
            .collect::<Vec<i32>>();
        (0..nums.len() - 1).rev().all(|i| {
            let diff = nums[i] - nums[i + 1] + 1;
            if diff > 0 {
                let p = primes.partition_point(|&x| x < diff);
                if p == primes.len() {
                    return false;
                }
                nums[i] -= primes[p];
            }
            nums[i] > 0
        })
    }
}

struct Solution {}
pub fn run() {
    let arr = vec![4, 9, 6, 10];
    tracing::info!("{}", Solution::prime_sub_operation(arr));
}
