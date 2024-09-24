#[derive(Default)]
struct TrieNode {
    children: [Option<Box<TrieNode>>; 10],
    is_end: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: Default::default(),
            is_end: false,
        }
    }
}

struct TrieTree {
    root: TrieNode,
}

impl TrieTree {
    pub fn new() -> Self {
        TrieTree {
            root: TrieNode::new(),
        }
    }

    pub fn add(&mut self, number: &str) {
        let mut current_node = &mut self.root;
        for ch in number.chars() {
            let index = ch.to_digit(10).expect("Invalid digit") as usize;
            current_node = current_node.children[index].get_or_insert(Box::new(TrieNode::new()));
        }
        current_node.is_end = true;
    }

    pub fn start_with(&self, prefix: &str) -> bool {
        let mut current_node = &self.root;
        for ch in prefix.chars() {
            let index = ch.to_digit(10).expect("Invalid digit") as usize;
            match &current_node.children[index] {
                Some(node) => current_node = node,
                None => return false,
            }
        }
        true
    }

    pub fn ends_with(&self, suffix: &str) -> bool {
        self.ends_with_helper(&self.root, suffix, String::new())
    }

    fn ends_with_helper(&self, node: &TrieNode, suffix: &str, current_str: String) -> bool {
        if node.is_end && current_str.ends_with(suffix) {
            return true;
        }

        for (index, child) in node.children.iter().enumerate() {
            if let Some(child_node) = child {
                let next_str = format!("{}{}", current_str, index);
                if self.ends_with_helper(node, suffix, next_str) {
                    return true;
                }
            }
        }
        false
    }
}

impl Solution {
    pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut trie1 = TrieTree::new();
        let mut trie2 = TrieTree::new();
        arr1.into_iter().for_each(|x| trie1.add(&x.to_string()));
        arr2.into_iter().for_each(|x| trie2.add(&x.to_string()));

        let mut max_depth = 0usize;
        let mut prefix = String::new();

        Self::dfs(&trie1, &trie2, &mut prefix, &mut max_depth);
        max_depth as i32
    }

    fn dfs(trie1: &TrieTree, trie2: &TrieTree, prefix: &mut String, max_depth: &mut usize) {
        if !trie1.start_with(prefix) || !trie2.start_with(prefix) {
            return;
        }

        *max_depth = *max_depth.max(&mut prefix.len());

        for digit in ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'] {
            prefix.push(digit);
            Self::dfs(trie1, trie2, prefix, max_depth);
            prefix.pop();
        }
    }
}

struct Solution {}
pub fn run() {
    tracing::info!(
        "{}",
        Solution::longest_common_prefix(vec![1, 10, 100], vec![1000])
    );
}
