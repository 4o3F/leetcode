use crate::utils::tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn recover_from_preorder(traversal: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut parser = traversal.bytes().peekable();
        let mut stack = Vec::new();
        let mut root_value = 0;
        while let Some(c) = parser.next_if(u8::is_ascii_digit) {
            root_value = root_value * 10 + (c - b'0') as i32;
        }

        stack.push((0, Rc::new(RefCell::new(TreeNode::new(root_value)))));

        loop {
            let mut depth = 0;
            while let Some(_) = parser.next_if_eq(&b'-') {
                depth += 1;
            }

            let mut value = 0;
            while let Some(c) = parser.next_if(u8::is_ascii_digit) {
                value = value * 10 + (c - b'0') as i32;
            }

            if value == 0 {
                return Some(stack.swap_remove(0).1);
            }

            loop {
                let last = stack.pop().unwrap();
                if last.0 < depth {
                    let new_node = Rc::new(RefCell::new(TreeNode::new(value)));
                    let mut last_borrow = last.1.borrow_mut();
                    if last_borrow.left.is_some() {
                        last_borrow.right = Some(new_node.clone());
                    } else {
                        last_borrow.left = Some(new_node.clone());
                    }
                    drop(last_borrow);
                    stack.push(last);
                    stack.push((depth, new_node));
                    break;
                }
            }
        }

    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{:?}",
        Solution::recover_from_preorder("1-2--3--4-5--6--7".to_string())
    )
}
