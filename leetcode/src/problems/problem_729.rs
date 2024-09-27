#[derive(Default)]
struct MyCalendar {
    time_map: std::collections::BTreeMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {
    fn new() -> Self {
        Self::default()
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        if let Some((_, &end_time)) = self.time_map.range(..=start).next_back() {
            if end_time > start {
                return false;
            }
        }

        if let Some((&start_time, _)) = self.time_map.range(start..).next() {
            if start_time < end {
                return false;
            }
        }

        self.time_map.insert(start, end);
        true
    }
}

pub fn run() {
    let mut calendar = MyCalendar::new();
    let mut result = vec![];
    result.push(calendar.book(10, 20));
    result.push(calendar.book(15, 25));
    result.push(calendar.book(20, 30));
    tracing::info!("{:?}", result);
}