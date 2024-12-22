use crate::utils::tree::TreeNode;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn reverse_odd_levels(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let Some(r) = root.clone() else { return root };
        let mut q = VecDeque::from([r]);
        let (mut i, mut vs) = (0, vec![]);
        while q.len() > 0 {
            let mut l = vec![];
            for _ in 0..q.len() {
                let n = q.pop_front().unwrap();
                let mut n = n.borrow_mut();
                if i % 2 > 0 && vs.len() > 0 {
                    n.val = vs.pop().unwrap();
                }
                if let Some(x) = n.left.clone() {
                    l.push(x.borrow().val);
                    q.push_back(x);
                }
                if let Some(x) = n.right.clone() {
                    l.push(x.borrow().val);
                    q.push_back(x);
                }
            }
            vs = l;
            i += 1
        }
        root
    }
}

struct Solution {}
pub fn run() {
    tracing::info!("{:?}", Solution::reverse_odd_levels(None));
}
