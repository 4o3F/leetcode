use std::collections::{BTreeMap, HashMap};

impl Solution {
    pub fn delete_duplicate_folder(paths: Vec<Vec<String>>) -> Vec<Vec<String>> {
        #[derive(Default)]
        struct Node {
            children: BTreeMap<String, Node>,
            deleted: bool,
        }

        fn insert_path(root: &mut Node, path: &[String]) {
            let mut current = root;
            for part in path {
                current = current.children.entry(part.clone()).or_default();
            }
        }

        fn encode(node: &mut Node, map: &mut HashMap<String, Vec<*mut Node>>) -> String {
            if node.children.is_empty() {
                return "()".to_string();
            }
            let mut parts = vec![];
            for (k, v) in &mut node.children {
                let sub = encode(v, map);
                parts.push(format!("{}{}", k, sub));
            }
            parts.sort();
            let sign = format!("({})", parts.concat());
            map.entry(sign.clone()).or_default().push(node as *mut _);
            sign
        }

        fn mark_deleted(map: HashMap<String, Vec<*mut Node>>) {
            for group in map.values() {
                if group.len() > 1 {
                    for &ptr in group {
                        unsafe {
                            (*ptr).deleted = true;
                        }
                    }
                }
            }
        }

        fn collect(node: &Node, path: &mut Vec<String>, result: &mut Vec<Vec<String>>) {
            for (k, v) in &node.children {
                if v.deleted {
                    continue;
                }
                path.push(k.clone());
                result.push(path.clone());
                collect(v, path, result);
                path.pop();
            }
        }

        let mut root = Node::default();
        for path in &paths {
            insert_path(&mut root, path);
        }

        let mut map = HashMap::new();
        encode(&mut root, &mut map);
        mark_deleted(map);

        let mut result = vec![];
        let mut path = vec![];
        collect(&root, &mut path, &mut result);
        result
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{:?}",
        Solution::delete_duplicate_folder(vec![
            vec!["a".to_string()],
            vec!["c".to_string()],
            vec!["d".to_string()],
            vec!["a".to_string(), "b".to_string()],
            vec!["c".to_string(), "b".to_string()],
            vec!["d".to_string(), "a".to_string()]
        ])
    )
}
