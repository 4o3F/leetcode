use std::cmp::Ordering::{Greater, Less};

impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let mut size = arr.len();
        let mut base = 0usize;

        while size > 1 {
            let half = size / 2;
            let mid = base + half;
            let cmp_after = unsafe { arr.get_unchecked(mid).cmp(arr.get_unchecked(mid + 1)) };
            let cmp_before = unsafe { arr.get_unchecked(mid - 1).cmp(arr.get_unchecked(mid)) };
            match (cmp_before, cmp_after) {
                (Less, Greater) => {
                    return mid as i32;
                }
                (Less, Less) => base = mid,
                (Greater, Less) => unreachable!(),
                _ => {}
            }

            size -= half;
        }

        unreachable!()
    }
}

struct Solution;

use utils::logger::init_logger;

fn main() {
    init_logger();
    tracing::info!(
        "{}",
        Solution::peak_index_in_mountain_array(vec![0, 3, 10, 5, 2])
    );
}
