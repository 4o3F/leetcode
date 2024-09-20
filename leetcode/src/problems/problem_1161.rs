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
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let mut result: Vec<i32> = Vec::new();
        let mut queue: VecDeque<(Rc<RefCell<TreeNode>>, usize)> = VecDeque::new();
        queue.push_back((root.unwrap(), 0));

        while let Some((node, depth)) = queue.pop_front() {
            if depth >= result.len() {
                result.push(node.borrow().val);
            } else {
                *result.get_mut(depth).unwrap() += node.borrow().val;
            }

            if let Some(left) = node.borrow().left.clone() {
                queue.push_back((left, depth + 1));
            }

            if let Some(right) = node.borrow().right.clone() {
                queue.push_back((right, depth + 1));
            }
        }

        (result
            .iter()
            .enumerate()
            .max_by(|(ia, a), (ib, b)| {
                match a.cmp(b) {
                    std::cmp::Ordering::Equal => ib.cmp(ia),
                    other => other,
                }
            })
            .map(|(index, _)| index)
            .unwrap()
            + 1) as i32
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
pub fn run() {
    let root = array_to_tree(vec![
        Some(1),
        Some(7),
        Some(0),
        Some(7),
        Some(-8),
        None,
        None,
    ]);
    tracing::info!("{}", Solution::max_level_sum(root));
    let root = array_to_tree(vec![
        Some(1),
        Some(1),
        Some(0),
        Some(7),
        Some(-8),
        Some(-7),
        Some(9)
    ]);
    tracing::info!("{}", Solution::max_level_sum(root));
}
