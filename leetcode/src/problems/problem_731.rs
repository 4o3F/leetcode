#[derive(Default)]
struct MyCalendarTwo {
    all_booking: Vec<(i32, i32)>,
    double_booking: Vec<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarTwo {
    fn new() -> Self {
        Self::default()
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        for (s, e) in &self.double_booking {
            if start >= *e || end <= *s {
                continue;
            } else {
                return false;
            }
        }

        for (s, e) in &self.all_booking {
            if start >= *e || end <= *s {
                continue;
            } else {
                self.double_booking.push((*s.max(&start), *e.min(&end)));
            }
        }

        self.all_booking.push((start, end));

        true
    }
}

/**
 * Your MyCalendarTwo object will be instantiated and called as such:
 * let obj = MyCalendarTwo::new();
 * let ret_1: bool = obj.book(start, end);
 */

pub fn run() {
    let mut obj = MyCalendarTwo::new();
    let ret_1 = obj.book(10, 20);
    let ret_2 = obj.book(50, 60);
    let ret_3 = obj.book(10, 40);
    let ret_4 = obj.book(5, 15);
    let ret_5 = obj.book(5, 10);
    let ret_6 = obj.book(25, 55);
    tracing::info!(
        "ret_1 = {}, ret_2 = {}, ret_3 = {}, ret_4 = {}, ret_5 = {}, ret_6 = {}",
        ret_1, ret_2, ret_3, ret_4, ret_5, ret_6
    );
}
