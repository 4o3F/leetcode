impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        let ranks = arr
            .iter()
            .copied()
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .enumerate()
            .map(|(i, v)| (v, i as i32 + 1))
            .collect::<std::collections::HashMap<_, _>>();
        arr.into_iter()
            .flat_map(|num| ranks.get(&num).copied())
            .collect()
    }
}

struct Solution {}

pub fn run() {
    let arr = vec![100,100,100];
    tracing::info!("{:?}", Solution::array_rank_transform(arr));
}
