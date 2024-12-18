impl Solution {
    fn find_discount(prices: &Vec<i32>, i: usize) -> i32 {
        for index in i + 1..prices.len() {
            if prices[index] <= prices[i] {
                return prices[index];
            }
        }
        0
    }

    pub fn final_prices(mut prices: Vec<i32>) -> Vec<i32> {
        for index in 0..prices.len() {
            prices[index] -= Solution::find_discount(&prices, index);
        }
        prices
    }
}

struct Solution {}
pub fn run() {
    tracing::info!("{:?}", Solution::final_prices(vec![8, 4, 6, 2, 3]));
}
