use std::{cell::RefCell, collections::VecDeque, rc::Rc};

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

pub fn array_to_tree(arr: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    if arr.is_empty() || arr[0].is_none() {
        return None;
    }

    let root = Rc::new(RefCell::new(TreeNode::new(arr[0].unwrap())));
    let mut q: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
    q.push_back(Rc::clone(&root));

    let mut i = 1;
    while i < arr.len() {
        let parent = match q.pop_front() {
            Some(p) => p,
            None => break,
        };

        // left
        if i < arr.len() {
            if let Some(v) = arr[i] {
                let left = Rc::new(RefCell::new(TreeNode::new(v)));
                parent.borrow_mut().left = Some(Rc::clone(&left));
                q.push_back(left);
            }
            i += 1;
        }

        // right
        if i < arr.len() {
            if let Some(v) = arr[i] {
                let right = Rc::new(RefCell::new(TreeNode::new(v)));
                parent.borrow_mut().right = Some(Rc::clone(&right));
                q.push_back(right);
            }
            i += 1;
        }
    }

    Some(root)
}
