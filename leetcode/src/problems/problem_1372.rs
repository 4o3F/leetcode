// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut longest = 0;
        let mut stack = Vec::new();
        stack.push((root, false, 0));
        while let Some((onode, left, len)) = stack.pop() {
            if let Some(node) = onode {
                stack.push((
                    node.borrow_mut().left.take(),
                    false,
                    if left { len + 1 } else { 1 },
                ));
                stack.push((
                    node.borrow_mut().right.take(),
                    true,
                    if left { 1 } else { len + 1 },
                ));
            } else {
                longest = longest.max(len - 1)
            }
        }
        longest
    }
}

struct Solution {}
pub fn array_to_tree(arr: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    fn build_tree(arr: &[Option<i32>], index: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if index < arr.len() {
            if let Some(val) = arr[index] {
                let node = Rc::new(RefCell::new(TreeNode::new(val)));
                let left_index = 2 * index + 1; // 左子树索引
                let right_index = 2 * index + 2; // 右子树索引
                node.borrow_mut().left = build_tree(arr, left_index);
                node.borrow_mut().right = build_tree(arr, right_index);
                Some(node)
            } else {
                None
            }
        } else {
            None
        }
    }

    build_tree(&arr, 0)
}

use tracing_unwrap::OptionExt;

pub fn run() {
    let root = array_to_tree(vec![
        Some(1),
        None,
        Some(1),
        Some(1),
        Some(1),
        None,
        None,
        Some(1),
        Some(1),
        None,
        Some(1),
        None,
        None,
        None,
        Some(1),
    ]);
    let result = Solution::longest_zig_zag(root);
    tracing::info!("{}", result);
}
