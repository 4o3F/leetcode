use crate::utils::tree::TreeNode;

use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};
use std::rc::Rc;

impl Solution {
    pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
        let Some(node) = root else {
            return -1;
        };
        let (mut queue, mut heap) = (VecDeque::from([node]), BinaryHeap::new());
        while queue.len() > 0 {
            let sum = (0..queue.len())
                .map(|_| {
                    let node = queue.pop_front().unwrap();
                    let node = node.borrow();
                    if let Some(left) = node.left.clone() {
                        queue.push_back(left);
                    }

                    if let Some(right) = node.right.clone() {
                        queue.push_back(right);
                    }
                    node.val as i64
                })
                .sum::<i64>();
            heap.push(Reverse(sum));
            if heap.len() > k as usize {
                heap.pop();
            }
        }
        if heap.len() == k as usize {
            heap.pop().unwrap().0
        } else {
            -1
        }
    }
}

struct Solution {}

pub fn run() {
    let root = vec![5, 8, 9, 2, 1, 3, 7, 4, 6]
        .into_iter()
        .map(Some)
        .collect::<Vec<Option<i32>>>();

    let root = crate::utils::tree::array_to_tree(root);
    tracing::info!("{:?}", Solution::kth_largest_level_sum(root, 2));
}
