use std::rc::Rc;
use std::{cell::RefCell, collections::HashMap};
use tracing_unwrap::OptionExt;
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
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        fn path_sum_helper(
            root: Option<Rc<RefCell<TreeNode>>>,
            target_sum: i32,
            curr_sum: i32,
            prefix_sum: &mut HashMap<i32, i32>,
            result: &mut i32,
        ) {
            if let Some(node) = root {
                let value = node.borrow().val;
                let curr = {
                    if let Some(_) = curr_sum.checked_add(value) {
                        curr_sum + value
                    } else {
                        prefix_sum.clear();
                        prefix_sum.insert(0, 1);
                        value
                    }
                };

                let prev = curr - target_sum;
                if let Some(num) = prefix_sum.get(&prev) {
                    *result += num;
                }
                prefix_sum.entry(curr).and_modify(|e| *e += 1).or_insert(1);

                path_sum_helper(
                    node.borrow_mut().left.take(),
                    target_sum,
                    curr,
                    prefix_sum,
                    result,
                );
                path_sum_helper(
                    node.borrow_mut().right.take(),
                    target_sum,
                    curr,
                    prefix_sum,
                    result,
                );

                prefix_sum.entry(curr).and_modify(|e| *e -= 1);
            }
        }
        let mut result = 0;
        let mut prefix_sum = HashMap::new();

        prefix_sum.insert(0, 1);
        path_sum_helper(root, target_sum, 0, &mut prefix_sum, &mut result);
        result
    }
}

// 新增函数：将数组转换为二叉树
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
        Some(10),
        Some(5),
        Some(-3),
        Some(3),
        Some(2),
        None,
        Some(11),
        Some(3),
        Some(-2),
        None,
        Some(1),
    ]);
    let target_sum = 8;
    let result = Solution::path_sum(root, target_sum);
    tracing::info!("{}", result);
}
