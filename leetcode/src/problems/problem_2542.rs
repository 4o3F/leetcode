use std::{cmp::Reverse, collections::BinaryHeap, iter::zip};

#[derive(Copy, Clone, Debug)]
struct Pair(i64, i64);

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.0.cmp(&other.0))
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl Eq for Pair {}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Solution {
    pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let mut sorted_nums = zip(
            nums1.into_iter().map(i64::from),
            nums2.into_iter().map(i64::from),
        )
        .collect::<Vec<_>>();
        sorted_nums.sort_by(|a, b| b.1.cmp(&a.1));
        let mut heap = BinaryHeap::<Reverse<Pair>>::new();
        let mut sum = 0i64;
        for pair in sorted_nums.iter().take(k as usize) {
            heap.push(Reverse(Pair(pair.0, pair.1)));
            sum += pair.0;
        }
        let mut result = sum * sorted_nums[k as usize - 1].1;
        // tracing::info!("{:?}", heap);
        // tracing::info!("{:?}", sorted_nums);
        // tracing::info!("{}", result);
        for pair in sorted_nums.iter().skip(k as usize) {
            // tracing::info!("Pair {} {}", pair.0, pair.1);
            if heap.peek().unwrap().0 .0 < pair.0 {
                // tracing::info!("Changed");
                sum += pair.0 - heap.peek().unwrap().0 .0;
                // tracing::info!("Changed to {} from {}", pair.0, heap.peek().unwrap().0.0);
                // tracing::info!("Sum become {}", sum);
                result = result.max(sum * pair.1);
                heap.pop();
                heap.push(Reverse(Pair(pair.0, pair.1)));
            }
        }
        result
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{}",
        Solution::max_score(vec![23, 16, 20, 7, 3], vec![19, 21, 22, 22, 12], 3)
    );

    // tracing::info!(
    //     "{}",
    //     Solution::max_score(vec![4, 2, 3, 1, 1], vec![7, 5, 10, 9, 6], 1)
    // );
}
