impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = head.as_mut();

        while let Some(current_node) = current.take()
            && let Some(next_node) = current_node.next.as_mut()
        {
            if current_node.val == next_node.val {
                current_node.next = next_node.next.take();
                current = Some(current_node);
            } else {
                current = current_node.next.as_mut();
            }
        }

        head
    }
}

struct Solution;

use utils::list::ListNode;
use utils::logger::init_logger;

fn main() {
    init_logger();
    let node = utils::list::vec2list(vec![1, 1, 2, 3, 3]);
    tracing::info!("{:?}", Solution::delete_duplicates(node));
}
