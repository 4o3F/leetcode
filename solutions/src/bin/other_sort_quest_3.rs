impl Solution {
    pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            Some(head) => {
                let mut unsorted = Some(head);
                let mut sorted: Option<Box<ListNode>> = None;

                while let Some(mut unsorted_node) = unsorted {
                    unsorted = unsorted_node.next.take();
                    let mut current_sorted_node_ref = &mut sorted;
                    while current_sorted_node_ref
                        .as_ref()
                        .is_some_and(|n| unsorted_node.val > n.val)
                    {
                        current_sorted_node_ref =
                            &mut current_sorted_node_ref.as_mut().unwrap().next;
                    }
                    unsorted_node.next = current_sorted_node_ref.take();
                    *current_sorted_node_ref = Some(unsorted_node);
                }

                sorted
            }
            None => None,
        }
    }
}

struct Solution;

use utils::list::ListNode;
use utils::logger::init_logger;

fn main() {
    init_logger();
    tracing::info!(
        "{:?}",
        Solution::insertion_sort_list(utils::list::vec2list(vec![4, 2, 1, 3]))
    );
}
