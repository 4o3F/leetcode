use std::collections::HashSet;

use itertools::Itertools;

impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        let poss = vec![
            "011237",
            "0122579",
            "012356789",
            "0124",
            "0134449",
            "0145678",
            "01466788",
            "0248",
            "0368888",
            "0469",
            "1",
            "112234778",
            "11266777",
            "122446",
            "125",
            "128",
            "1289",
            "13468",
            "16",
            "2",
            "224588",
            "23",
            "23334455",
            "234455668",
            "23678",
            "256",
            "35566",
            "4",
            "46",
            "8",
        ]
        .into_iter()
        .map(|x| x.to_string())
        .collect::<HashSet<_>>();

        let n = n.to_string().chars().sorted().collect::<String>();

        poss.contains(&n)
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::reordered_power_of2(10))
}
//  {"011237","0122579","012356789","0124","0134449", "0145678","01466788","0248","0368888","0469","1","112234778","11266777","122446","125","128","1289","13468","16","2","224588","23","23334455","234455668","23678","256","35566","4","46","8"}
