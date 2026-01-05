use std::collections::hash_map::Entry::Occupied;
use std::collections::HashMap;
impl Solution {
    pub fn query_results(_limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut balls = HashMap::<i32, i32>::new();
        let mut result = Vec::<i32>::new();
        let mut colors = HashMap::<i32, i32>::new();
        for query in queries {
            let (ball, color) = (query[0], query[1]);
            // tracing::info!("{:?} {:?}", balls, colors);
            if *balls.entry(ball).or_insert(0) == 0 {
                colors.entry(color).and_modify(|v| *v += 1).or_insert(1);
                balls.entry(ball).and_modify(|c| *c = color);
            } else {
                match colors.entry(*balls.get(&ball).unwrap()) {
                    Occupied(mut occupied_entry) => {
                        if *occupied_entry.get() == 1 {
                            occupied_entry.remove();
                        } else {
                            *occupied_entry.get_mut() -= 1;
                        }
                    }
                    _ => unreachable!(),
                }

                colors.entry(color).and_modify(|v| *v += 1).or_insert(1);
                balls.entry(ball).and_modify(|c| *c = color);
            }

            result.push(colors.len() as i32);
        }

        result
    }
}

struct Solution;
fn main() {
    use utils::prelude::*;
    init_logger();
    let queries = vec![[0, 1], [0, 4], [0, 4], [0, 1], [1, 2]];
    let queries = queries.into_iter().map(|x| x.to_vec()).collect::<Vec<_>>();
    tracing::info!("{:?}", Solution::query_results(1, queries));
}
