use std::collections::VecDeque;

impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        // value, index
        let mut stack = VecDeque::<(i32, usize)>::with_capacity(prices.len());

        let mut result = prices.clone();
        for (idx, &price) in prices.iter().enumerate() {
            while let Some((top_price, top_idx)) = stack.pop_back() {
                if price <= top_price {
                    result[top_idx] -= price;
                } else {
                    stack.push_back((top_price, top_idx));
                    break;
                }
            }
            stack.push_back((price, idx));
        }
        result
    }
}

struct Solution;

use utils::logger::init_logger;

fn main() {
    init_logger();
    tracing::info!("{:?}", Solution::final_prices(vec![8, 4, 6, 2, 3]));
    tracing::info!("{:?}", Solution::final_prices(vec![10, 1, 1, 6]));
}
