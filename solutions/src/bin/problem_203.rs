use utils::{list::ListNode, logger::init_logger};

impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut walker = &mut head;

        loop {
            match walker {
                Some(node) if node.val == val => {
                    *walker = node.next.take();
                }
                Some(node) => {
                    walker = &mut node.next;
                }
                None => break,
            }
        }
        head
    }
}

struct Solution;

fn main() {
    init_logger();
    let head = utils::list::vec2list(vec![1, 2, 6, 3, 4, 5, 6]);
    tracing::info!("{:?}", Solution::remove_elements(head, 6))
}
