use crate::utils::tree::{array_to_tree, TreeNode};

use std::{cell::RefCell, collections::HashSet, rc::Rc};

struct FindElements {
    values: HashSet<i32>,
}

macro_rules! node_as_ref {
    ($node:expr) => {
        $node
            .as_ref()
            .map(|root| root.borrow())
            .as_ref()
            .map(|umm| &**umm)
    };
}

impl FindElements {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut values = HashSet::new();

        fn dfs_set_node(node: Option<&TreeNode>, value: i32, values: &mut HashSet<i32>) {
            let Some(node) = node else {
                return;
            };

            values.insert(value);
            dfs_set_node(node_as_ref!(node.left), value * 2 + 1, values);
            dfs_set_node(node_as_ref!(node.right), value * 2 + 2, values);
        }
        dfs_set_node(node_as_ref!(root), 0, &mut values);

        Self { values }
    }

    fn find(&self, target: i32) -> bool {
        self.values.contains(&target)
    }
}

pub fn run() {
    let obj = FindElements::new(array_to_tree(vec![Some(-1), None, Some(-1)]));
    tracing::info!("{}", obj.find(1));
    tracing::info!("{}", obj.find(2));
}
