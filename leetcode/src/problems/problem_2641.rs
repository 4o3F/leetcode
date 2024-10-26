use crate::utils::tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn replace_value_in_tree(
        mut root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(mut node) = root.as_mut().map(|n| n.borrow_mut()) {
            node.val = 0;
        } else {
            return None;
        }
        let mut level_sums = Vec::new();
        Self::dfs_sums(0, root.as_ref(), &mut level_sums);
        Self::dfs_update(0, root.as_mut(), &level_sums);
        root
    }

    fn dfs_sums(depth: usize, node: Option<&Rc<RefCell<TreeNode>>>, ds: &mut Vec<i32>) {
        let Some(node) = node.as_ref().map(|n| n.borrow()) else {
            return;
        };
        if let Some(v) = ds.get_mut(depth) {
            *v += node.val;
        } else {
            ds.push(node.val);
        }
        Self::dfs_sums(depth + 1, node.left.as_ref(), ds);
        Self::dfs_sums(depth + 1, node.right.as_ref(), ds);
    }

    fn dfs_update(depth: usize, node: Option<&mut Rc<RefCell<TreeNode>>>, ds: &Vec<i32>) {
        let Some(mut node) = node.map(|n| n.borrow_mut()) else {
            return;
        };
        let left_val = node.left.as_ref().map_or(0, |n| n.borrow().val);
        let right_val = node.right.as_ref().map_or(0, |n| n.borrow().val);
        let up = ds.get(depth + 1).unwrap_or(&0) - left_val - right_val;
        if let Some(mut left) = node.left.as_mut().map(|n| n.borrow_mut()) {
            left.val = up;
        }
        if let Some(mut right) = node.right.as_mut().map(|n| n.borrow_mut()) {
            right.val = up;
        }
        Self::dfs_update(depth + 1, node.left.as_mut(), ds);
        Self::dfs_update(depth + 1, node.right.as_mut(), ds);
    }
}

struct Solution {}

pub fn run() {
    let root = crate::utils::tree::array_to_tree(vec![
        Some(5),
        Some(4),
        Some(9),
        Some(1),
        Some(10),
        None,
        Some(7),
    ]);
    tracing::info!("{:?}", Solution::replace_value_in_tree(root));
}
