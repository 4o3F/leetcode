impl Solution {
    pub fn divide_players(skill: Vec<i32>) -> i64 {
        let mut skill = skill;
        skill.sort_unstable();
        let mut right_index = skill.len() - 1;
        let mut left_index = 0;

        let sum = skill[left_index] + skill[right_index];
        let mut team: Vec<(i32, i32)> = Vec::new();
        while left_index < right_index {
            if skill[left_index] + skill[right_index] == sum {
                team.push((skill[left_index], skill[right_index]));
                left_index += 1;
                right_index -= 1;
            } else {
                return -1;
            }
        }

        let mut total = 0i64;
        for (a, b) in team {
            let a = a as i64;
            let b = b as i64;
            total += a * b;
        }
        total as i64
    }
}

struct Solution {}
pub fn run() {
    let arr = vec![3, 2, 5, 1, 3, 4];
    tracing::info!("{}", Solution::divide_players(arr));
    let arr = vec![3, 4];
    tracing::info!("{}", Solution::divide_players(arr));
    let arr = vec![1, 1, 2, 3];
    tracing::info!("{}", Solution::divide_players(arr));
}
