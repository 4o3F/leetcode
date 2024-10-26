use crate::utils::tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root == p || root == q {
            root
        } else if let Some(node) = root {
            let left =
                Self::lowest_common_ancestor(node.borrow().left.clone(), p.clone(), q.clone());
            let right = Self::lowest_common_ancestor(node.borrow().right.clone(), p, q);
            if left.is_some() && right.is_some() {
                Some(node)
            } else if left.is_none() {
                right
            } else {
                left
            }
        } else {
            None
        }
    }
}

struct Solution {}

pub fn run() {
    let root = crate::utils::tree::array_to_tree(vec![
        Some(3),
        Some(5),
        Some(1),
        Some(6),
        Some(2),
        Some(0),
        Some(8),
        None,
        None,
        Some(7),
        Some(4),
    ]);

    let p = crate::utils::tree::array_to_tree(vec![Some(5)]);
    let q = crate::utils::tree::array_to_tree(vec![Some(1)]);
    tracing::info!("{:?}", Solution::lowest_common_ancestor(root, p, q));
}
