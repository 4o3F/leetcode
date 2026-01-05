use std::collections::{BTreeSet, HashMap};

impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i64> {
        type LL = i64;
        type Pii = (i32, i32);
        let (k, x) = (k as usize, x as usize);
        let n = nums.len();
        let mut res = vec![0i64; n - k + 1];
        let mut freq: HashMap<i32, i32> = HashMap::new();
        let mut active: BTreeSet<Pii> = BTreeSet::new();
        let mut bank: BTreeSet<Pii> = BTreeSet::new();
        let mut sum: LL = 0;

        fn upd_all(sum: &mut LL, active: &mut BTreeSet<Pii>, bank: &mut BTreeSet<Pii>, x: usize) {
            if bank.is_empty() {
                return;
            }
            let mx = *bank.iter().next_back().unwrap();
            let mv_opt = active.iter().next().cloned();
            if active.len() < x || mv_opt.is_none_or(|mv| mx > mv) {
                *sum += mx.0 as LL * mx.1 as LL;
                active.insert(mx);
                bank.remove(&mx);
                if active.len() > x {
                    let mv = mv_opt.unwrap();
                    *sum -= mv.0 as LL * mv.1 as LL;
                    bank.insert(mv);
                    active.remove(&mv);
                }
            }
        }

        fn upd_val(
            val: i32,
            delta: i32,
            sum: &mut LL,
            freq: &mut HashMap<i32, i32>,
            active: &mut BTreeSet<Pii>,
            bank: &mut BTreeSet<Pii>,
            x: usize,
        ) {
            let old = *freq.get(&val).unwrap_or(&0);
            let new = old + delta;
            freq.insert(val, new);
            let og = (old, val);
            let ng = (new, val);

            if active.remove(&og) {
                *sum -= og.0 as LL * og.1 as LL;
                active.insert(ng);
                *sum += ng.0 as LL * ng.1 as LL;
            } else {
                bank.remove(&og);
                bank.insert(ng);
            }
            upd_all(sum, active, bank, x);
        }

        for &v in nums.iter().take(k) {
            upd_val(v, 1, &mut sum, &mut freq, &mut active, &mut bank, x);
        }
        res[0] = sum;

        for i in k..n {
            upd_val(nums[i], 1, &mut sum, &mut freq, &mut active, &mut bank, x);
            upd_val(
                nums[i - k],
                -1,
                &mut sum,
                &mut freq,
                &mut active,
                &mut bank,
                x,
            );
            res[i - k + 1] = sum;
        }

        res
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{:?}",
        Solution::find_x_sum(vec![1, 1, 2, 2, 3, 4, 2, 3], 6, 2)
    )
}
