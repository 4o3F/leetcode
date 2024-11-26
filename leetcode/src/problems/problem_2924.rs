impl Solution {
    pub fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let result = edges.into_iter().fold(
            (vec![0; n as usize], n),
            |(mut in_degrees, mut zero_count), edge| {
                if in_degrees[edge[1] as usize] == 0 {
                    zero_count -= 1;
                }
                in_degrees[edge[1] as usize] += 1;
                (in_degrees, zero_count)
            },
        );
        if result.1 != 1 {
            return -1;
        } else {
            return result
                .0
                .into_iter()
                .enumerate()
                .filter(|(_, in_degree)| *in_degree == 0)
                .collect::<Vec<(usize, i32)>>()[0].0 as i32;
        }
    }
}

struct Solution {}
pub fn run() {
    let edges = vec![vec![0, 1], vec![1, 2]];
    tracing::info!("{}", Solution::find_champion(3, edges));
    let edges = vec![vec![0, 2], vec![1, 3], vec![1, 2]];
    tracing::info!("{}", Solution::find_champion(4, edges));
}
