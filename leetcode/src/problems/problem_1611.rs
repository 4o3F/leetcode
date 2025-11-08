impl Solution {
    pub fn minimum_one_bit_operations(n: i32) -> i32 {
        let s = format!("{:b}", n);

        let mut m = 0;

        let mut p1 = 0;
        let mut p2 = 0;

        let mut t1 = 0;
        let mut t2;

        for c in s.chars().rev() {
            if c == '1' {
                t1 = std::cmp::min(p1 + 2 * m + 1, p2 + m + 1);
                t2 = std::cmp::min(p1, p2 + m);
            } else {
                t1 = p1;
                t2 = p2 + m + 1;
            }

            p1 = t1;
            p2 = t2;

            m = 2 * m + 1;
        }

        t1
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::minimum_one_bit_operations(3))
}
