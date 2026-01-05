impl Solution {
    pub fn maximum_beauty(mut items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        items.sort_unstable();
        items.dedup_by(|right, left| right[1] <= left[1]);
        queries
            .into_iter()
            .map(|q| match items.partition_point(|it| it[0] <= q) {
                0 => 0,
                i => items[i - 1][1],
            })
            .collect()
    }
}

struct Solution {}
fn main() {
    use utils::prelude::*;
    init_logger();
    let items = vec![[1, 2], [3, 2], [2, 4], [5, 6], [3, 5]]
        .into_iter()
        .map(|slice| slice.to_vec())
        .collect();
    let queries = vec![1, 2, 3, 4, 5, 6];
    tracing::info!("{:?}", Solution::maximum_beauty(items, queries))
}
