use utils::tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn construct_from_pre_post(
        preorder: Vec<i32>,
        postorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match preorder.len() {
            0 => None,
            1 => Some(Rc::new(RefCell::new(TreeNode::new(preorder[0])))),
            n => {
                let root = preorder[0];
                let first = preorder[1];
                let idx = postorder.iter().position(|&x| x == first).unwrap();
                let mut out = TreeNode::new(root);
                out.left = Self::construct_from_pre_post(
                    preorder[1..=idx + 1].to_vec(),
                    postorder[0..=idx].to_vec(),
                );
                out.right = Self::construct_from_pre_post(
                    preorder[idx + 2..].to_vec(),
                    postorder[idx + 1..n - 1].to_vec(),
                );
                Some(Rc::new(RefCell::new(out)))
            }
        }
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{:?}",
        Solution::construct_from_pre_post(vec![1, 2, 4, 5, 3, 6, 7], vec![4, 5, 2, 6, 7, 3, 1])
    );
}
