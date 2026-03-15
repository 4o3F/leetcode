use std::collections::HashMap;

struct SegmentTree {
    tree: Vec<i32>,
    length: usize,
}

impl SegmentTree {
    pub fn new(arr: &[i32]) -> Self {
        let n = arr.len();
        let mut tree = SegmentTree {
            tree: vec![0; 4 * n],
            length: n,
        };
        Self::build(&mut tree, arr, 0, 0, n - 1);
        tree
    }

    fn build(&mut self, arr: &[i32], node: usize, start: usize, end: usize) {
        if start == end {
            self.tree[node] = arr[start];
        } else {
            let mid = (start + end) / 2;
            Self::build(self, arr, 2 * node + 1, start, mid);
            Self::build(self, arr, 2 * node + 2, mid + 1, end);
            self.tree[node] = self.tree[node * 2 + 1] + self.tree[node * 2 + 2];
        }
    }

    fn update(
        &mut self,
        node: usize,
        start: usize,
        end: usize,
        update_idx: usize,
        update_val: i32,
    ) {
        if start == end {
            self.tree[node] = update_val;
        } else {
            let mid = (start + end) / 2;
            if update_idx <= mid {
                self.update(2 * node + 1, start, mid, update_idx, update_val);
            } else {
                self.update(2 * node + 2, mid + 1, end, update_idx, update_val);
            }
            self.tree[node] = self.tree[2 * node + 1] + self.tree[2 * node + 2];
        }
    }

    fn query(
        &self,
        node: usize,
        start: usize,
        end: usize,
        left_bound: usize,
        right_bound: usize,
    ) -> i32 {
        if right_bound < start || left_bound > end {
            return 0;
        }

        if start >= left_bound && end <= right_bound {
            return self.tree[node];
        }

        let mid = (start + end) / 2;
        self.query(node * 2 + 1, start, mid, left_bound, right_bound)
            + self.query(node * 2 + 2, mid + 1, end, left_bound, right_bound)
    }

    pub fn point_update(&mut self, update_idx: usize, update_val: i32) {
        self.update(0, 0, self.length - 1, update_idx, update_val);
    }

    pub fn range_query(&self, left_bound: usize, right_bound: usize) -> i32 {
        self.query(0, 0, self.length - 1, left_bound, right_bound)
    }
}

impl Solution {
    pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
        let mut all_values = Vec::<i64>::with_capacity(nums.len() * 2);
        for &num in nums.iter() {
            all_values.push(num as i64);
            all_values.push(num as i64 * 2);
        }

        all_values.sort();
        all_values.dedup();

        let value2index = all_values
            .iter()
            .enumerate()
            .map(|(idx, &value)| (value, idx))
            .collect::<HashMap<i64, usize>>();

        let mut segment_tree = SegmentTree::new(&vec![0; all_values.len()]);
        let mut result = 0;
        for &num2 in nums.iter() {
            let target = num2 as i64 * 2;
            let target_index = value2index.get(&target).unwrap() + 1;
            if target_index < all_values.len() {
                result += segment_tree.range_query(target_index, all_values.len() - 1);
            }

            let num2_idx = value2index.get(&(num2 as i64)).unwrap();
            let current = segment_tree.range_query(*num2_idx, *num2_idx);
            segment_tree.point_update(*num2_idx, current + 1);
        }
        result
    }
}

struct Solution;

use utils::logger::init_logger;

fn main() {
    init_logger();
    tracing::info!("{}", Solution::reverse_pairs(vec![1, 3, 2, 3, 1]));
}
