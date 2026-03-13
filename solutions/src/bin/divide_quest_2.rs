use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build(&inorder, &postorder)
    }

    fn build(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder.is_empty() {
            return None;
        }
        let mut root = TreeNode::new(*postorder.last().unwrap());
        let split = inorder.split(|&x| x == root.val).collect::<Vec<_>>();
        root.left = Self::build(split[0], &postorder[..split[0].len()]);
        root.right = Self::build(split[1], &postorder[split[0].len()..postorder.len() - 1]);
        Some(Rc::new(RefCell::new(root)))
    }
}

struct Solution;

use utils::logger::init_logger;
use utils::tree::TreeNode;

fn main() {
    init_logger();
    tracing::info!(
        "{:?}",
        Solution::build_tree(vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3])
    );
}
