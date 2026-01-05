use utils::tree::TreeNode;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        if root.is_none() {
            return result;
        }

        let mut queue: VecDeque<(Rc<RefCell<TreeNode>>, usize)> = VecDeque::new();
        queue.push_back((root.unwrap(), 0));
        while let Some((node, depth)) = queue.pop_front() {
            if depth >= result.len() {
                result.push(node.borrow().val);
            }

            if let Some(right) = node.borrow().right.clone() {
                queue.push_back((right, depth + 1));
            }

            if let Some(left) = node.borrow().left.clone() {
                queue.push_back((left, depth + 1));
            }
        }

        result
    }
}

struct Solution {}

fn main() {
    use utils::prelude::*;
    init_logger();
    let root = utils::tree::array_to_tree(vec![
        Some(1),
        Some(2),
        Some(3),
        None,
        Some(5),
        None,
        Some(4),
    ]);
    tracing::info!("{:?}", Solution::right_side_view(root));
}
