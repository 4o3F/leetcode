impl Solution {
    pub fn leftmost_building_queries(heights: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let queries_len = queries.len();
        let mut ans: Vec<i32> = vec![-1; queries_len];

        let nontrivial_q: Vec<Vec<(i32, usize)>> = queries
            .into_iter()
            .enumerate()
            .map(|(query_i, q)| {
                let tuple = if q[0] > q[1] {
                    (q[1] as usize, q[0] as usize, query_i)
                } else {
                    (q[0] as usize, q[1] as usize, query_i)
                };
                if heights[tuple.1] > heights[tuple.0] || tuple.0 == tuple.1 {
                    ans[query_i] = tuple.1 as i32;
                }
                tuple
            })
            .filter(|(a, b, _)| heights[*a] >= heights[*b] && *a != *b)
            .fold(vec![Vec::new(); heights.len()], |mut q, (a, b, query_i)| {
                q[b].push((heights[a], query_i));
                q
            });

        let mut mono_stack: Vec<(i32, i32)> = Vec::new();
        nontrivial_q
            .into_iter()
            .enumerate()
            .rev()
            .for_each(|(height_i, entries)| {
                let height_i = height_i as i32;
                entries
                    .into_iter()
                    .map(|(height, query_i)| {
                        if mono_stack.is_empty() {
                            (query_i, -1)
                        } else {
                            let position = mono_stack.partition_point(|&(h, _)| h > height);
                            if position > 0 {
                                (query_i, mono_stack[position - 1].1)
                            } else {
                                (query_i, -1)
                            }
                        }
                    })
                    .for_each(|(query_i, a)| {
                        if a >= 0 {
                            ans[query_i] = a;
                        }
                    });
                while mono_stack
                    .last()
                    .is_some_and(|&(h, _)| h <= heights[height_i as usize])
                {
                    mono_stack.pop();
                }
                mono_stack.push((heights[height_i as usize], height_i));
            });
        ans
    }
}

struct Solution {}

pub fn run() {
    tracing::info!(
        "{:?}",
        Solution::leftmost_building_queries(
            vec![6, 4, 8, 5, 2, 7],
            vec![vec![0, 1], vec![0, 3], vec![2, 4], vec![3, 4], vec![2, 2]]
        )
    )
}
