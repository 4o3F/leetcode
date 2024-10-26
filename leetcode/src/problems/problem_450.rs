use crate::utils::tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
macro_rules! b {
    ($n:expr) => {
        $n.as_ref().unwrap().borrow()
    };
}
macro_rules! bmut {
    ($n:expr) => {
        $n.as_ref().unwrap().borrow_mut()
    };
}

impl Solution {
    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }

        if key > b!(root).val {
            let temp = Self::delete_node(b!(root).right.clone(), key);
            bmut!(root).right = temp;
        } else if key < b!(root).val {
            let temp = Self::delete_node(b!(root).left.clone(), key);
            bmut!(root).left = temp;
        } else {
            if b!(root).left.is_none() {
                return b!(root).right.clone();
            } else if b!(root).right.is_none() {
                return b!(root).left.clone();
            } else {
                let mut curr = b!(root).left.clone();
                while b!(curr).right.is_some() {
                    curr = curr.unwrap().borrow().right.clone();
                }
                bmut!(root).val = b!(curr).val;
                let temp = Self::delete_node(b!(root).left.clone(), b!(root).val);
                bmut!(root).left = temp;
            }
        }
        root
    }
}

struct Solution {}

pub fn run() {
    let root = crate::utils::tree::array_to_tree(vec![
        Some(5),
        Some(3),
        Some(6),
        Some(2),
        Some(4),
        None,
        Some(7),
    ]);
    tracing::info!("{:?}", Solution::delete_node(root, 3));
}
