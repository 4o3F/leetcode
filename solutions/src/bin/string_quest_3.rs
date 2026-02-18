impl Solution {
    pub fn mask_pii(s: String) -> String {
        if s.contains('@') {
            // Email
            let Some((name, domain)) = s.split_once('@') else {
                unreachable!()
            };
            let (name, domain) = (name.to_lowercase(), domain.to_lowercase());
            let mut iter = name.chars();
            format!(
                "{}*****{}@{}",
                iter.next().unwrap(),
                iter.last().unwrap(),
                domain
            )
        } else {
            // Phone
            let number = s.chars().filter(|x| x.is_numeric()).collect::<String>();
            let length = number.len();
            let number = &number[length - 4..length];

            match length - 10 {
                0 => format!("***-***-{}", number),
                1 => format!("+*-***-***-{}", number),
                2 => format!("+**-***-***-{}", number),
                3 => format!("+***-***-***-{}", number),
                _ => unreachable!(),
            }
        }
    }
}

struct Solution;

use utils::logger::init_logger;

fn main() {
    init_logger();
    tracing::info!(
        "{}",
        Solution::mask_pii("LeetCode@LeetCode.com".to_string())
    );
        tracing::info!(
        "{}",
        Solution::mask_pii("1(234)567-890".to_string())
    );
}
