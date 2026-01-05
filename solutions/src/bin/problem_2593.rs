use std::collections::BinaryHeap;

struct Item {
    num: i64,
    index: usize,
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.index == other.index
    }
}

impl Eq for Item {}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.num == other.num {
            self.index.cmp(&other.index).reverse()
        } else {
            self.num.cmp(&other.num).reverse()
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn find_score(nums: Vec<i32>) -> i64 {
        let mut marked_index = vec![false; nums.len()];
        let mut heap: BinaryHeap<Item> =
            BinaryHeap::from_iter(nums.iter().enumerate().map(|(index, &num)| Item {
                num: i64::from(num),
                index,
            }));
        let mut result = 0;
        while !heap.is_empty() {
            let item = heap.pop().unwrap();
            let (num, index) = (item.num, item.index);
            if marked_index[index] {
                continue;
            }
            marked_index[index] = true;
            if index > 0 {
                marked_index[index - 1] = true;
            }
            if index < nums.len() - 1 {
                marked_index[index + 1] = true;
            }
            result += num;
            // tracing::debug!("{} {}", index, num);
            // tracing::debug!("{:?}", marked_index);
        }
        result
    }
}

struct Solution {}
fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::find_score(vec![5, 1, 1, 7, 2, 4]));
    tracing::info!("{}", Solution::find_score(vec![2, 3, 5, 1, 3, 2]));
    tracing::info!("{}", Solution::find_score(vec![2, 1, 3, 4, 5, 2]));
}
