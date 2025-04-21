impl Solution {
    pub fn number_of_arrays(differences: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let (mut min, mut max) = (0, 0);
        let mut before = 0i32;
        for difference in differences {
            let next = before.saturating_add(difference);
            min = min.min(next);
            max = max.max(next);
            before = next;
        }

        // tracing::info!("min {} max {}", min, max);
        ((upper - lower) - (max - min) + 1).max(0)
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{}",
        Solution::number_of_arrays(
            vec![
                100000, 100000, 100000, 100000, 100000, 100000, 100000, 100000, 100000, 100000,
                100000, 100000, 100000, 100000, 100000, 100000, 100000, 100000, 100000, 100000,
                100000, 100000, 100000, 100000, 100000, 100000, 100000, 100000, 100000, 100000,
                100000, 100000, 100000, 100000, 100000, 100000, 100000, 100000, 100000, 100000,
                100000, 100000
            ],
            -100000,
            100000
        )
    );
    tracing::info!("{}", Solution::number_of_arrays(vec![-40], -46, 53));
    tracing::info!("{}", Solution::number_of_arrays(vec![1, -3, 4], 1, 6));
    tracing::info!(
        "{}",
        Solution::number_of_arrays(vec![3, -4, 5, 1, -2], -4, 5)
    );
    tracing::info!("{}", Solution::number_of_arrays(vec![4, -7, 2], 3, 6));
}
