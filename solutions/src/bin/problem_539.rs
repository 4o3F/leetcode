struct Solution {}

impl Solution {
    fn parse_time(time: &str) -> (i32, i32) {
        let mut time = time.split(":");
        let hour = time.next().unwrap().parse::<i32>().unwrap();
        let minute = time.next().unwrap().parse::<i32>().unwrap();
        (hour, minute)
    }

    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        let mut time_points = time_points.clone();
        time_points.sort();
        let mut result = i32::MAX;
        time_points.push(time_points.first().unwrap().clone());
        for window in time_points.windows(2) {
            let (start, end) = (Self::parse_time(&window[0]), Self::parse_time(&window[1]));
            let mut interval = end.1 - start.1 + 60 * (end.0 - start.0);
            if interval < 0 {
                interval += 60 * 24;
            }
            result = result.min(interval);
        }

        result
    }
}

fn main() {
    use utils::prelude::*;
    init_logger();
    let time_points = ["00:00", "23:59", "00:00"];
    let time_points: Vec<String> = time_points.iter().map(|s| s.to_string()).collect();
    tracing::info!("{}", Solution::find_min_difference(time_points));
}
