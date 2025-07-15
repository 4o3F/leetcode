use crate::utils::list::ListNode;

impl Solution {
    pub fn get_decimal_value(mut head: Option<Box<ListNode>>) -> i32 {
        let mut res = 0;
        while let Some(item) = head {
            res *= 2;
            res += item.val;
            head = item.next;
        }
        res
    }
}

struct Solution;
pub fn run() {
    tracing::info!(
        "{}",
        Solution::get_decimal_value(crate::utils::list::vec2list(vec![1, 0, 1]))
    )
}
