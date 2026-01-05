impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        (0..1 << nums.len())
            .fold((0, 0), |(count, max_bo), combo| {
                let Some(bitwise_or) = nums
                    .iter()
                    .enumerate()
                    .filter_map(|(index, num)| ((combo >> index) & 1 == 1).then_some(*num))
                    .reduce(|acc, num| acc | num)
                else {
                    return (count, max_bo);
                };
                match bitwise_or.cmp(&max_bo) {
                    std::cmp::Ordering::Less => (count, max_bo),
                    std::cmp::Ordering::Equal => (count + 1, max_bo),
                    std::cmp::Ordering::Greater => (1, bitwise_or),
                }
            })
            .0
    }
}

struct Solution {}
fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::count_max_or_subsets(vec![3, 2, 1, 5]));
}
