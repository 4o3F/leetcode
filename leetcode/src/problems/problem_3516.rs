impl Solution {
    pub fn find_closest(x: i32, y: i32, z: i32) -> i32 {
        match (x - z).abs().cmp(&(y - z).abs()) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 2,
        }
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::find_closest(2, 7, 4))
}
