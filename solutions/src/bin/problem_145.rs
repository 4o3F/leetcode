use utils::logger::init_logger;
use utils::tree::{TreeNode, array_to_tree};

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::<i32>::new();
        // Node, left visited, right visited
        let mut stack = Vec::<(Option<Rc<RefCell<TreeNode>>>, bool, bool)>::new();
        stack.push((root, false, false));
        while let Some((root, left_visited, right_visited)) = stack.pop() {
            let Some(node_rc) = root else { continue };
            if right_visited {
                result.push(node_rc.borrow().val);
                continue;
            }

            if !left_visited {
                stack.push((Some(node_rc.clone()), true, false));
                stack.push((node_rc.borrow().left.clone(), false, false));
            } else {
                stack.push((Some(node_rc.clone()), true, true));
                stack.push((node_rc.borrow().right.clone(), false, false));
            }
        }

        result
    }
}

struct Solution;

fn main() {
    init_logger();
    let root = array_to_tree(vec![Some(1), None, Some(2), Some(3)]);
    tracing::info!("{:?}", Solution::postorder_traversal(root))
}
