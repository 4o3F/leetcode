use utils::logger::init_logger;
use utils::tree::{self, TreeNode};

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::<i32>::new();
        let mut stack = Vec::<Option<Rc<RefCell<TreeNode>>>>::new();
        stack.push(root);

        while let Some(root) = stack.pop() {
            if let Some(pnode) = root {
                let node = pnode.borrow();
                result.push(node.val);
                stack.push(node.right.clone());
                stack.push(node.left.clone());
            }
        }
        result
    }
}

struct Solution;

fn main() {
    init_logger();
    let root = tree::array_to_tree(vec![Some(1), None, Some(2), Some(3)]);
    tracing::info!("{:?}", Solution::preorder_traversal(root))
}
