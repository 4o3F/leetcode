#[derive(Debug)]
struct Item {
    pass: i32,
    total: i32,
}

impl Item {
    fn calc_change_ratio(&self) -> f64 {
        (f64::from(self.pass + 1) / f64::from(self.total + 1))
            - (f64::from(self.pass) / f64::from(self.total))
    }

    fn clac_pass_ratio(&self) -> f64 {
        f64::from(self.pass) / f64::from(self.total)
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.calc_change_ratio()
            .partial_cmp(&other.calc_change_ratio())
            .unwrap()
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        (self.calc_change_ratio() - other.calc_change_ratio()).abs() < f64::EPSILON
    }
}

impl Eq for Item {}

impl Solution {
    pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        let mut pq = std::collections::BinaryHeap::new();
        classes.into_iter().for_each(|class| {
            pq.push(Item {
                pass: class[0],
                total: class[1],
            });
        });
        for _ in 0..extra_students {
            let item = pq.pop().unwrap();
            pq.push(Item {
                pass: item.pass + 1,
                total: item.total + 1,
            });
        }
        let result = pq
            .into_vec()
            .into_iter()
            .fold((0.0, 0), |(acc, len), item| {
                // tracing::debug!("{:?} {}", item, item.clac_pass_ratio());
                (acc + item.clac_pass_ratio(), len + 1)
            });
        result.0 / f64::from(result.1)
    }
}

struct Solution {}
fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::max_average_ratio(vec![vec![1, 2], vec![3, 5], vec![2, 2]], 2)
    );
}
