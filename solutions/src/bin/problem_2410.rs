impl Solution {
    pub fn match_players_and_trainers(mut players: Vec<i32>, mut trainers: Vec<i32>) -> i32 {
        players.sort_unstable();
        trainers.sort_unstable();
        players
            .into_iter()
            .scan(trainers.into_iter(), |t, p| t.find(|&t| p <= t))
            .count() as _
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::match_players_and_trainers(vec![4, 7, 9], vec![8, 2, 5, 8])
    )
}
