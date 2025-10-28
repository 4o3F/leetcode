impl Solution {
    pub fn next_beautiful_number(n: i32) -> i32 {
        let mut unit = 1;
        let mut count = 1;
        while unit * 10 <= n {
            unit *= 10;
            count += 1;
        }
        let mut v: Vec<i32> = (0..7).collect();
        Self::backtrack(count, unit, n + 1, &mut v)
            .or(Self::backtrack(count + 1, unit * 10, 0, &mut v))
            .unwrap()
    }

    fn backtrack(mut count: i32, unit: i32, n: i32, v: &mut Vec<i32>) -> Option<i32> {
        if unit == 0 {
            if count == 0 {
                return Some(0);
            }
        } else {
            for i in 1.max(n / unit)..7 {
                if (v[i as usize] == i && i <= count) || v[i as usize] != 0 {
                    if v[i as usize] == i {
                        count -= i;
                    }
                    v[i as usize] -= 1;
                    let n2 = if i * unit > n { 0 } else { n % unit };
                    if let Some(ans2) = Self::backtrack(count, unit / 10, n2, v) {
                        return Some(i * unit + ans2);
                    }
                    v[i as usize] += 1;
                    if v[i as usize] == i {
                        count += i;
                    }
                }
            }
        }
        None
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::next_beautiful_number(1))
}
