use utils::logger::init_logger;

impl Solution {
    pub fn reverse_bits(n: i32) -> i32 {
        n.reverse_bits()
    }
}

struct Solution;

fn main() {
    init_logger();
    tracing::info!("{}", Solution::reverse_bits(43261596));
}
