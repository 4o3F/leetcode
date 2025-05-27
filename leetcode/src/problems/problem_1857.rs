impl Solution {
    pub fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32 {
        let node_count = colors.len();
        let mut adj_matrix: Vec<Vec<i32>> = vec![vec![]; node_count];
        edges
            .iter()
            .for_each(|edge| adj_matrix[edge[0] as usize].push(edge[1]));

        let mut appeared = vec![false; node_count];
        let mut color_counts = vec![vec![0; 26]; node_count];
        let mut in_stack = vec![false; node_count];
        let colors = colors
            .chars()
            .map(|x| x as usize - 'a' as usize)
            .collect::<Vec<_>>();

        let ans = (0..node_count).fold(0, |acc, idx| {
            acc.max(Solution::dfs(
                idx,
                &colors,
                &adj_matrix,
                &mut appeared,
                &mut color_counts,
                &mut in_stack,
            ))
        });
        if ans == i32::MAX {
            -1
        } else {
            ans
        }
    }

    fn dfs(
        node: usize,
        colors: &Vec<usize>,
        adj_matrix: &Vec<Vec<i32>>,
        appeared: &mut Vec<bool>,
        color_counts: &mut Vec<Vec<i32>>,
        in_stack: &mut Vec<bool>,
    ) -> i32 {
        if in_stack[node] {
            return i32::MAX;
        }

        if appeared[node] {
            return color_counts[node][colors[node]];
        }

        appeared[node] = true;
        in_stack[node] = true;

        for adj in adj_matrix[node].iter() {
            if Solution::dfs(
                *adj as usize,
                colors,
                adj_matrix,
                appeared,
                color_counts,
                in_stack,
            ) == i32::MAX
            {
                return i32::MAX;
            }
            (0..26).for_each(|i| {
                color_counts[node][i] = color_counts[node][i].max(color_counts[*adj as usize][i])
            });
        }

        in_stack[node] = false;
        color_counts[node][colors[node]] += 1;
        color_counts[node][colors[node]]
    }
}

struct Solution;

pub fn run() {
    let edges = vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![3, 4]];
    tracing::info!(
        "{}",
        Solution::largest_path_value("abaca".to_string(), edges)
    )
}
