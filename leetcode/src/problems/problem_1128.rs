use std::collections::HashMap;

impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut frequence = HashMap::<[i32; 2], i32>::new();
        for dominoe in dominoes {
            let (mut a, mut b) = (dominoe[0], dominoe[1]);
            if a > b {
                std::mem::swap(&mut a, &mut b);
            }
            let entry = frequence.entry([a, b]).or_insert(0);
            *entry += 1;
        }

        frequence.values().fold(0, |mut result, freq| {
            if *freq >= 2 {
                result += freq * (freq - 1) / 2;
            }
            result
        })
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{}",
        Solution::num_equiv_domino_pairs(vec![vec![1, 2], vec![2, 1], vec![3, 4], vec![5, 6]])
    )
}
