impl Solution {
    #[inline]
    fn check(
        min_power: &i64,
        st: &Vec<i32>,
        mem: &mut Vec<i32>,
        r: &usize,
        mut power: i64,
        mut k: i64,
        n: &usize,
    ) -> bool {
        for i in 0..(*n) {
            if power < *min_power {
                let need = *min_power - power;

                if need > k {
                    return false;
                }

                k -= need;
                let pos = (*n - 1).min(i + *r);
                mem[pos] = need as i32;
                power += need;
            }

            if i >= *r {
                power -= st[i - *r] as i64;
                power -= mem[i - *r] as i64;
                mem[i - *r] = 0;
            }

            if i + *r + 1 < *n {
                power += st[i + *r + 1] as i64;
            }
        }

        true
    }

    pub fn max_power(stations: Vec<i32>, r: i32, k: i32) -> i64 {
        let n = stations.len();
        let r_usize = r as usize;
        let k_ll = k as i64;
        let power = stations[0..(r_usize + 1)]
            .iter()
            .map(|x| (*x) as i64)
            .sum::<i64>();

        let mut right = power
            + stations[(r_usize + 1)..n]
                .iter()
                .map(|x| (*x) as i64)
                .sum::<i64>()
            + k_ll;
        let mut left = 0_i64;
        let mut ans = 0_i64;

        let mut mem = vec![0_i32; n];

        while left <= right {
            let mid = left + (right - left) / 2;

            if Self::check(&mid, &stations, &mut mem, &r_usize, power, k_ll, &n) {
                ans = mid;
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        ans
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::max_power(vec![1, 2, 4, 5, 0], 1, 2));
}
