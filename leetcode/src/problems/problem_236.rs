use tracing_unwrap::OptionExt;
struct Solution {}

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

    let p = array_to_tree(vec![Some(5)]);
    let q = array_to_tree(vec![Some(1)]);
    tracing::info!("{:?}", Solution::lowest_common_ancestor(root, p, q));
}
