impl Solution {
    pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = head.as_mut();
        let mut odd_values = Vec::new();
        let mut even_values = Vec::new();
        let mut idx = 1;
        while let Some(current_node) = current.take() {
            if idx % 2 == 0 {
                even_values.push(current_node.val);
            } else {
                odd_values.push(current_node.val);
            }
            current = current_node.next.as_mut();
            idx += 1;
        }
        odd_values.append(&mut even_values);
        let mut current = head.as_mut();
        for val in odd_values {
            let Some(current_node) = current.take() else {
                unreachable!()
            };
            current_node.val = val;
            current = current_node.next.as_mut();
        }
        head
    }
}

struct Solution;

use utils::list::ListNode;
use utils::logger::init_logger;

fn main() {
    init_logger();
    tracing::info!(
        "{:?}",
        Solution::odd_even_list(utils::list::vec2list(vec![1, 2, 3, 4, 5]))
    );
}
