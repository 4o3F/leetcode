impl Solution {
    pub fn min_end(n: i32, x: i32) -> i64 {
        let mut shifts = Vec::new();
        let mut cur = x as i64;
        let mut calc = n as i64 - 1;

        for i in 0..32 {
            if (x & (1 << i)) == 0 {
                shifts.push(i);
            }
        }
        // tracing::debug!("shifts: {:?}", shifts);

        for i in 32..64 {
            shifts.push(i);
        }

        let mut i = 0;
        while calc > 0 {
            // tracing::debug!("{}", cur);
            // tracing::debug!("{}",(calc & 1));
            cur += (calc & 1) << shifts[i];
            calc >>= 1;
            i += 1;
        }

        cur
    }
}

struct Solution {}
pub fn run() {
    // tracing::info!("{}", Solution::min_end(3, 4));
    // tracing::info!("{}", Solution::min_end(2, 7));
    tracing::info!("{}", Solution::min_end(6, 1));
}
