use crate::utils::tree::TreeNode;

use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;
type NodePointer = Rc<RefCell<TreeNode>>;
type NodeOption = Option<NodePointer>;

impl Solution {
    fn gen_subtree_depths(node: &NodeOption, depths: &mut Vec<i32>) -> i32 {
        if let Some(node) = node {
            let node = node.as_ref().borrow();
            let left_depth = Self::gen_subtree_depths(&node.left, depths);
            let right_depth = Self::gen_subtree_depths(&node.right, depths);
            if depths.len() <= node.borrow().val as usize {
                depths.resize((node.borrow().val + 1) as usize, 0);
            }
            depths[node.borrow().val as usize] = left_depth.max(right_depth) + 1;
            depths[node.borrow().val as usize]
        } else {
            0
        }
    }

    fn gen_max_depth(
        node: &NodeOption,
        current_node_depth: i32,
        current_node_max_depth: i32,
        max_depths: &mut Vec<i32>,
        depths: &Vec<i32>,
    ) {
        if let Some(node) = node {
            let node = node.as_ref().borrow();
            max_depths[node.val as usize] = current_node_max_depth;

            let left_depth = node
                .left
                .as_ref()
                .map_or(0, |x| depths[x.as_ref().borrow().val as usize]);
            let right_depth = (&node.right)
                .as_ref()
                .map_or(0, |x| depths[x.as_ref().borrow().val as usize]);

            // tracing::info!(
            //     "Node {} depth {} max_depth {} LD {} RD {}",
            //     node.val,
            //     current_node_depth,
            //     current_node_max_depth,
            //     left_depth,
            //     right_depth
            // );

            Self::gen_max_depth(
                &node.left,
                current_node_depth + 1,
                current_node_max_depth.max(right_depth + current_node_depth),
                max_depths,
                depths,
            );

            Self::gen_max_depth(
                &node.right,
                current_node_depth + 1,
                current_node_max_depth.max(left_depth + current_node_depth),
                max_depths,
                depths,
            );
        }
    }

    pub fn tree_queries(root: NodeOption, queries: Vec<i32>) -> Vec<i32> {
        let mut depths = Vec::new();
        Self::gen_subtree_depths(&root, &mut depths);

        let mut max_depths = vec![0; depths.len()];

        // tracing::info!("{:?}", depths);
        Self::gen_max_depth(&root, 0, 0, &mut max_depths, &depths);

        // tracing::info!("{:?}", max_depths);

        queries
            .into_iter()
            .map(|x| max_depths[x as usize])
            .collect()
    }
}

struct Solution {}
pub fn run() {
    let arr = vec![
        1,
        3,
        4,
        2,
        i32::MAX,
        6,
        5,
        i32::MAX,
        i32::MAX,
        i32::MAX,
        i32::MAX,
        i32::MAX,
        i32::MAX,
        i32::MAX,
        7,
    ];
    let arr = arr
        .into_iter()
        .map(|x| if x == i32::MAX { None } else { Some(x) })
        .collect::<Vec<_>>();

    let tree = crate::utils::tree::array_to_tree(arr);
    tracing::info!("{:?}", Solution::tree_queries(tree, vec![4]));
}
