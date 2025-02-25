use std::cmp::Ordering::{Equal, Greater, Less};

struct ProblemContext<'a> {
    bob: usize,
    amount: &'a [i32],
    tree: &'a [Vec<usize>],
}

impl Solution {
    pub fn most_profitable_path(edges: Vec<Vec<i32>>, bob: i32, amount: Vec<i32>) -> i32 {
        let n = amount.len();
        let mut tree = vec![vec![]; n];

        // Construct adjacency list
        for edge in &edges {
            let (u, v) = (edge[0] as usize, edge[1] as usize);
            tree[u].push(v);
            tree[v].push(u);
        }

        let ctx = ProblemContext {
            bob: bob as usize,
            amount: &amount,
            tree: &tree,
        };

        // Start DFS from the root (0), parent set as usize::MAX (invalid parent)
        let (alice_profit, _) = Self::dp_rec(0, usize::MAX, 0, &ctx);
        alice_profit
    }

    // Recursive DFS to compute Alice's max profit and Bob's distance from `node`
    fn dp_rec(
        node: usize,
        parent: usize,
        current_depth: usize,
        ctx: &ProblemContext,
    ) -> (i32, usize) {
        let bob_init_depth = if node == ctx.bob { 0 } else { ctx.tree.len() };

        // If `node` is a leaf (not the root), handle Alice's profit correctly
        if ctx.tree[node].len() == 1 && current_depth != 0 {
            let alice_profit = if node == ctx.bob { 0 } else { ctx.amount[node] };
            return (alice_profit, bob_init_depth);
        }

        // Explore child nodes recursively and find:
        //  - Maximum profit Alice can collect (`alice_profit`)
        //  - Minimum distance Bob needs to reach this node (`bob_distance`)
        let (alice_profit, bob_distance) = ctx.tree[node]
            .iter()
            .filter(|&&neighbor| neighbor != parent) // Avoid backtracking to parent
            .map(|&neighbor| Self::dp_rec(neighbor, node, current_depth + 1, ctx))
            .fold((i32::MIN, bob_init_depth), |(a_profit, b_dist), (a, b)| {
                (a_profit.max(a), b_dist.min(b + 1))
            });

        // Adjust Alice's profit based on when Bob arrives at this node
        let adjusted_profit = match current_depth.cmp(&bob_distance) {
            Less => alice_profit + ctx.amount[node], // Alice arrives first → full amount
            Equal => alice_profit + ctx.amount[node] / 2, // Alice & Bob arrive together → split amount
            Greater => alice_profit,                      // Bob arrives first → Alice gets nothing
        };

        (adjusted_profit, bob_distance)
    }
}
struct Solution;

pub fn run() {
    tracing::info!(
        "{}",
        Solution::most_profitable_path(
            vec![
                vec![0, 1],
                vec![0, 2],
                vec![1, 3],
                vec![1, 4],
                vec![1, 5],
                vec![2, 6]
            ],
            6,
            vec![8, 9, 1, 2, 3, 4, 5]
        )
    );
}
