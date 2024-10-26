use crate::utils::tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn flip_equiv(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let Some(r1) = root1 else {
            return root2.is_none();
        };

        let Some(r2) = root2 else {
            return false;
        };
        let (r1, r2) = (r1.borrow(), r2.borrow());
        r1.val == r2.val
            && (Self::flip_equiv(r1.left.clone(), r2.left.clone())
                && Self::flip_equiv(r1.right.clone(), r2.right.clone())
                || Self::flip_equiv(r1.left.clone(), r2.right.clone())
                    && Self::flip_equiv(r1.right.clone(), r2.left.clone()))
    }
}

struct Solution {}

pub fn run() {
    let root1 = vec![1, 2, 3, 4, 5, 6, i32::MAX, i32::MAX, i32::MAX, 7, 8];
    let root1 = root1
        .into_iter()
        .map(|x| if x == i32::MAX { None } else { Some(x) })
        .collect::<Vec<_>>();

    let root2 = vec![
        1,
        3,
        2,
        i32::MAX,
        6,
        4,
        5,
        i32::MAX,
        i32::MAX,
        i32::MAX,
        i32::MAX,
        8,
        7,
    ];
    let root2 = root2
        .into_iter()
        .map(|x| if x == i32::MAX { None } else { Some(x) })
        .collect::<Vec<_>>();
    tracing::info!(
        "{:?}",
        Solution::flip_equiv(crate::utils::tree::array_to_tree(root1), crate::utils::tree::array_to_tree(root2))
    );
}
