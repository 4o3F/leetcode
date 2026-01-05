use utils::logger::init_logger;
use utils::tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(root) = root {
            let left = &root.borrow().left;
            let right = &root.borrow().right;
            let left_depth = Self::depth(left);
            let right_depth = Self::depth(right);
            left_depth.abs_diff(right_depth) < 2
                && Self::is_balanced(left.clone())
                && Self::is_balanced(right.clone())
        } else {
            true
        }
    }

    fn depth(root: &Option<Rc<RefCell<TreeNode>>>) -> usize {
        if let Some(root) = root {
            let left = &root.borrow().left;
            let right = &root.borrow().right;
            Self::depth(left).max(Self::depth(right)) + 1
        } else {
            0
        }
    }
}

struct Solution;

fn main() {
    init_logger();
    let tree = utils::tree::array_to_tree(vec![
        Some(3),
        Some(9),
        Some(20),
        None,
        None,
        Some(15),
        Some(7),
    ]);
    tracing::info!("Test1 {}", Solution::is_balanced(tree));
}
