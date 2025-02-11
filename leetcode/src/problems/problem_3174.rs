impl Solution {
    pub fn clear_digits(s: String) -> String {
        let chars = s.chars().collect::<Vec<char>>();
        let mut is_removed = vec![false; chars.len()];
        for i in 0..chars.len() {
            if chars[i].is_digit(10) {
                is_removed[i] = true;
                let (mut j, mut flag) = (i - 1, false);
                loop {
                    if !is_removed[j] {
                        flag = true;
                        break;
                    } else {
                        if j == 0 {
                            break;
                        }
                        j -= 1;
                    }
                }
                if flag {
                    is_removed[j] = true;
                } else {
                    is_removed[i] = false;
                }
            }
            tracing::info!("{:?}", is_removed);
        }
        chars
            .into_iter()
            .enumerate()
            .filter(|(i, _)| !is_removed[*i])
            .map(|(_, c)| c)
            .collect::<String>()
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::clear_digits("abc".to_string()));
    tracing::info!("{}", Solution::clear_digits("cb34".to_string()));
}
