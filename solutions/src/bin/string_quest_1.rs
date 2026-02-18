enum State {
    Unknown,
    AllLow,
    FirstUp,
    OnlyFirstUp,
    AllUp,
}

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let mut state = State::Unknown;
        for c in word.chars() {
            match state {
                State::Unknown => {
                    if c.is_lowercase() {
                        state = State::AllLow;
                    } else {
                        state = State::FirstUp;
                    }
                }
                State::AllLow => {
                    if c.is_uppercase() {
                        return false;
                    }
                }
                State::FirstUp => {
                    if c.is_lowercase() {
                        state = State::OnlyFirstUp;
                    } else {
                        state = State::AllUp;
                    }
                }
                State::OnlyFirstUp => {
                    if c.is_uppercase() {
                        return false;
                    }
                }
                State::AllUp => {
                    if c.is_lowercase() {
                        return false;
                    }
                }
            }
        }
        true
    }
}

struct Solution;

use utils::logger::init_logger;

fn main() {
    init_logger();
    tracing::info!("{}", Solution::detect_capital_use("FlaG".to_string()));
}
