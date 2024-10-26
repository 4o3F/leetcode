use crate::utils::tree::TreeNode;

use std::rc::Rc;
use std::{cell::RefCell, collections::HashMap};

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

struct Solution {}

pub fn run() {
    let root = crate::utils::tree::array_to_tree(vec![
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
