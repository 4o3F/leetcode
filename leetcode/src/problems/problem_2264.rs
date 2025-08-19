impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        let chars: Vec<char> = num.chars().collect();
        chars.windows(3).fold(String::new(), |mut str, window| {
            if window[0] == window[1] && window[1] == window[2] {
                let new = window.iter().collect::<String>();
                if str.is_empty() {
                    str = new;
                } else if new.parse::<i32>().unwrap() > str.parse::<i32>().unwrap() {
                    str = new;
                }
            }
            str
        })
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{}",
        Solution::largest_good_integer("6777133339".to_string())
    );
}
