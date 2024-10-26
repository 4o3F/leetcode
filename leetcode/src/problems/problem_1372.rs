use crate::utils::tree::TreeNode;

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

pub fn run() {
    let root = crate::utils::tree::array_to_tree(vec![
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
