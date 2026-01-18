use utils::logger::init_logger;
use utils::tree::{array_to_tree, TreeNode};

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        let mut queue = VecDeque::<(Rc<RefCell<TreeNode>>, i32)>::new();
        let mut result = false;
        if let Some(root) = root {
            queue.push_back((Rc::clone(&root), root.borrow().val));
        } else {
            return false;
        }

        while let Some((root, current_sum)) = queue.pop_front() {
            let left_child = &root.borrow().left;
            let right_child = &root.borrow().right;
            if left_child.is_none() && right_child.is_none() && current_sum == target_sum {
                result = true;
                break;
            }

            if let Some(left_child) = left_child {
                queue.push_back((left_child.clone(), current_sum + left_child.borrow().val));
            }
            if let Some(right_child) = right_child {
                queue.push_back((right_child.clone(), current_sum + right_child.borrow().val));
            }
        }
        result
    }
}

struct Solution;

fn main() {
    init_logger();
    let root = array_to_tree(vec![
        Some(5),
        Some(4),
        Some(8),
        Some(11),
        None,
        Some(13),
        Some(4),
        Some(7),
        Some(2),
        None,
        None,
        None,
        Some(1),
    ]);
    tracing::info!("{}", Solution::has_path_sum(root, 22));
    let root = array_to_tree(vec![Some(1), Some(2), Some(3)]);
    tracing::info!("{}", Solution::has_path_sum(root, 5));
}
