use std::collections::HashSet;

impl Solution {
    pub fn minimum_teachings(n: i32, languages: Vec<Vec<i32>>, friendships: Vec<Vec<i32>>) -> i32 {
        fn has_common_element(a: &[i32], b: &[i32]) -> bool {
            b.iter().any(|item| a.contains(item))
        }

        let mut need_to_teach = HashSet::new();
        for friendship in friendships.iter() {
            let user1 = friendship[0] - 1;
            let user2 = friendship[1] - 1;
            if !has_common_element(&languages[user1 as usize], &languages[user2 as usize]) {
                need_to_teach.insert(user1);
                need_to_teach.insert(user2);
            }
        }

        if need_to_teach.is_empty() {
            return 0;
        }

        let mut lang_map = vec![0; n as usize];
        for user in need_to_teach.iter() {
            for lang in languages[*user as usize].iter() {
                lang_map[(lang - 1) as usize] += 1;
            }
        }

        let (k, _) = lang_map.iter().enumerate().max_by_key(|&(_, v)| v).unwrap();
        let common_lang = k as i32 + 1;

        let mut num_users = 0;
        for user in need_to_teach.iter() {
            if !languages[*user as usize].contains(&common_lang) {
                num_users += 1;
            }
        }

        num_users
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::minimum_teachings(
            2,
            vec![vec![1], vec![2], vec![1, 2]],
            vec![vec![1, 2], vec![1, 3], vec![2, 3]]
        )
    );
}
