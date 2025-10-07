use std::collections::{BTreeSet, HashMap};
use std::ops::Bound::*;

impl Solution {
    pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
        let mut dry_days = BTreeSet::<usize>::new();
        let mut lake_full_day = HashMap::new();
        let mut result = vec![1; rains.len()];
        for (day, rain_lake) in rains.into_iter().enumerate() {
            if rain_lake == 0 {
                // No rain this day
                dry_days.insert(day);
                continue;
            }
            if lake_full_day.contains_key(&rain_lake) {
                // If the lake is already full and rained again
                if let Some(clear_day) = dry_days
                    .range((Excluded(lake_full_day[&rain_lake]), Excluded(day)))
                    .next()
                    .copied()
                {
                    result[clear_day] = rain_lake;
                    dry_days.remove(&clear_day);
                } else {
                    return Vec::new();
                }
            }

            lake_full_day.insert(rain_lake, day);
            result[day] = -1;
        }
        result
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{:?}", Solution::avoid_flood(vec![1, 2, 3, 4]))
}
