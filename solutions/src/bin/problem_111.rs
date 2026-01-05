use utils::logger::init_logger;
use utils::tree::{array_to_tree, TreeNode};

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // tracing::info!("{:?}", root);
        if root.is_none() {
            1
        } else {
            Self::dfs(&root)
        }
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root) = root {
            // tracing::info!("{}", root.borrow().val.clone());
            let left_node = root.borrow().left.clone();
            let right_node = root.borrow().right.clone();
            if left_node.is_none() && right_node.is_none() {
                0
            } else {
                Self::dfs(&left_node).min(Self::dfs(&right_node)) + 1
            }
        } else {
            i32::MAX
        }
    }
}

struct Solution;

fn main() {
    init_logger();
    let tree = array_to_tree(vec![
        Some(2),
        None,
        Some(3),
        None,
        Some(4),
        None,
        Some(5),
        None,
        Some(6),
    ]);
    tracing::info!("{}", Solution::min_depth(tree));
}
