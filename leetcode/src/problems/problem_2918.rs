impl Solution {
    pub fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let (num1_zero, num1_sum) = nums1.iter().fold((0i64, 0i64), |(zero_count, sum), num| {
            let num = i64::from(*num);
            match num {
                0 => (zero_count + 1, sum + 1),
                num => (zero_count, sum + num),
            }
        });
        let (num2_zero, num2_sum) = nums2.iter().fold((0i64, 0i64), |(zero_count, sum), num| {
            let num = i64::from(*num);
            match num {
                0 => (zero_count + 1, sum + 1),
                num => (zero_count, sum + num),
            }
        });

        if num1_sum == num2_sum {
            return num1_sum;
        }

        if num1_sum > num2_sum {
            // Can't match
            if num2_zero == 0 {
                return -1;
            } else {
                return num1_sum;
            }
        } else {
            if num1_zero == 0 {
                return -1;
            } else {
                return num2_sum;
            }
        }
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::min_sum(vec![3, 2, 0, 1, 0], vec![6, 5, 0]));
}
