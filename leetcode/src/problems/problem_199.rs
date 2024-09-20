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
struct Solution {}

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

pub fn run() {
    let root = array_to_tree(vec![
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
