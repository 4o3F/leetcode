use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use utils::tree::TreeNode;
impl Solution {
    pub fn minimum_operations(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let Some(r) = root else {
            return 0;
        };
        let (mut q, mut r) = (VecDeque::from([r]), 0);
        while !q.is_empty() {
            let mut s = vec![];
            for _ in 0..q.len() {
                let n = q.pop_front().unwrap();
                let n = n.borrow();
                s.push(n.val);
                if let Some(left) = n.left.clone() {
                    q.push_back(left);
                }
                if let Some(right) = n.right.clone() {
                    q.push_back(right);
                }
            }
            let mut ix: Vec<_> = (0..s.len()).collect();
            ix.sort_unstable_by_key(|&i| s[i]);
            for i in 0..ix.len() {
                while ix[i] != i {
                    let t = ix[i];
                    ix.swap(i, t);
                    r += 1;
                }
            }
        }
        r
    }
}

struct Solution {}

fn main() {
    use utils::prelude::*;
    init_logger();
    let root = utils::tree::array_to_tree(vec![
        Some(1),
        Some(4),
        Some(3),
        Some(7),
        Some(6),
        Some(8),
        Some(5),
        None,
        None,
        None,
        None,
        Some(9),
        None,
        Some(10),
    ]);
    tracing::info!("{}", Solution::minimum_operations(root));
}
