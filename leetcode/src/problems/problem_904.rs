impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let (mut fruit1_type, mut window_start, mut fruit2_start) = (-1, 0, 0);
        fruits
            .iter()
            .enumerate()
            .map(|(idx, &current_type)| {
                if current_type != fruits[fruit2_start] {
                    if current_type != fruit1_type {
                        window_start = fruit2_start
                    }
                    fruit1_type = fruits[fruit2_start];
                    fruit2_start = idx
                }
                idx - window_start + 1
            })
            .max()
            .unwrap() as _
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::total_fruit(vec![0, 1, 2, 2]))
}
