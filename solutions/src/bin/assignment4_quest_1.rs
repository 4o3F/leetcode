impl Solution {
    fn month2number(month: &str) -> &str {
        match month {
            "Jan" => "01",
            "Feb" => "02",
            "Mar" => "03",
            "Apr" => "04",
            "May" => "05",
            "Jun" => "06",
            "Jul" => "07",
            "Aug" => "08",
            "Sep" => "09",
            "Oct" => "10",
            "Nov" => "11",
            "Dec" => "12",
            _ => unreachable!(),
        }
    }

    fn day2number(day: &str) -> String {
        let day = day
            .chars()
            .filter(|x| x.is_numeric())
            .collect::<String>()
            .parse::<u32>()
            .unwrap();
        format!("{:0>2}", day)
    }

    pub fn reformat_date(date: String) -> String {
        let Some((day, other)) = date.split_once(" ") else {
            unreachable!()
        };
        let Some((month, year)) = other.split_once(" ") else {
            unreachable!()
        };
        format!(
            "{}-{}-{}",
            year,
            Self::month2number(month),
            Self::day2number(day)
        )
    }
}

struct Solution;

use utils::logger::init_logger;

fn main() {
    init_logger();
    tracing::info!("{}", Solution::reformat_date("20th Oct 2052".to_string()));
}
