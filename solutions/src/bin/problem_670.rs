impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        let mut order1 = 10;
        let mut maxval = num;

        while order1 <= num {
            let mut order2 = 1;
            let digit1 = num / order1 % 10;

            while order1 > order2 {
                let digit2 = num / order2 % 10;

                maxval = maxval.max(
                    num - digit1 * order1 - digit2 * order2 + digit1 * order2 + digit2 * order1,
                );

                order2 *= 10;
            }
            order1 *= 10;
        }
        maxval
    }
}

struct Solution {}
fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::maximum_swap(2736));
}
