impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
        potions.sort_unstable();
        spells
            .into_iter()
            .map(|spell| {
                let partition =
                    potions.partition_point(|&potion| (spell as i64 * potion as i64) < success);
                potions.len() as i32 - partition as i32
            })
            .collect::<Vec<_>>()
    }
}

struct Solution;
fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{:?}",
        Solution::successful_pairs(vec![5, 1, 3], vec![1, 2, 3, 4, 5], 7)
    );
}
