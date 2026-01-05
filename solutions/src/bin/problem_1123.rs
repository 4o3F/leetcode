use utils::tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn solve(node: Option<Rc<RefCell<TreeNode>>>) -> (i32, Option<Rc<RefCell<TreeNode>>>) {
        if node.is_none() {
            return (0, None);
        }

        if let Some(r) = node.clone() {
            let rb = r.borrow();
            let (h1, l1) = Self::solve(rb.left.clone());
            let (h2, l2) = Self::solve(rb.right.clone());
            if h1 > h2 {
                return (h1 + 1, l1);
            }
            if h2 > h1 {
                return (h2 + 1, l2);
            }
            return (h1 + 1, node);
        }
        unreachable!()
    }

    pub fn lca_deepest_leaves(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::solve(root).1
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    let root = array_to_tree(vec![
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
    tracing::info!("{:?}", Solution::lca_deepest_leaves(root));
}
