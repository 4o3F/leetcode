impl Solution {
    pub fn product_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        const MODULO: i64 = 1_000_000_007;

        let array = (0..32)
            .filter(|x| (n >> x) & 1 == 1)
            .map(|pow| i64::from(2_i32.pow(pow)))
            .collect::<Vec<_>>();

        // tracing::info!("{:?}", array);
        queries
            .iter()
            .map(|query| {
                // tracing::info!("query {:?}", query);
                let mut result = 1_i64;
                for idx in query[0]..=query[1] {
                    result *= array[idx as usize];
                    result %= MODULO;
                    // tracing::info!("{}", result);
                }
                result as i32
            })
            .collect()
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{:?}",
        Solution::product_queries(919, vec![vec![0, 6], vec![2, 2], vec![0, 3]])
    )
}
