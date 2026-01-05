use itertools::Itertools;

impl Solution {
    pub fn maximum_total_damage(p: Vec<i32>) -> i64 {
        let (mut d, mut m) = (vec![(0, 0)], 0);
        p.iter()
            .sorted()
            .chunk_by(|&x| x)
            .into_iter()
            .map(|(&v, g)| {
                while !d.is_empty() && d[0].1 + 2 < v as i64 {
                    m = d.remove(0).0.max(m)
                }
                d.push((v as i64 * g.count() as i64 + m, v as i64));
                d[d.len() - 1].0
            })
            .max()
            .unwrap()
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::maximum_total_damage(vec![1, 1, 3, 4]))
}
