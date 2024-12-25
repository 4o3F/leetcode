use crate::utils::tree::TreeNode;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::i32;
use std::rc::Rc;

impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return Vec::new();
        }
        let mut queue = VecDeque::from([root.unwrap()]);
        let mut result = Vec::new();
        while !queue.is_empty() {
            let mut max = i32::MIN;
            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap();
                max = max.max(node.borrow().val);
                if let Some(left) = node.borrow().left.clone() {
                    queue.push_back(left);
                };
                if let Some(right) = node.borrow().right.clone() {
                    queue.push_back(right);
                };
            }
            result.push(max);
        }

        result
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{:?}",
        Solution::largest_values(crate::utils::tree::array_to_tree(vec![
            Some(1),
            Some(3),
            Some(2),
            Some(5),
            Some(3),
            None,
            Some(9)
        ]))
    );
}
