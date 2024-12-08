#[derive(Debug)]
struct Event {
    start: i32,
    end: i32,
    value: i32,
    suffix_max: i32,
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.start.cmp(&other.start))
    }
}

impl Eq for Event {}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end
    }
}

impl Solution {
    pub fn max_two_events(events: Vec<Vec<i32>>) -> i32 {
        let mut events = Vec::from_iter(events.into_iter().map(|event| Event {
            start: event[0],
            end: event[1],
            value: event[2],
            suffix_max: 0,
        }));
        events.sort_unstable();
        let mut events = events
            .into_iter()
            .rev()
            .scan(0, |max_suffix, mut event| {
                *max_suffix = (*max_suffix).max(event.value);
                event.suffix_max = *max_suffix;
                Some(event)
            })
            .collect::<Vec<_>>();
        events.reverse();
        // tracing::debug!("events: {:?}", events);
        let mut result = 0;
        for index in 0..events.len() - 1 {
            let mut low = index + 1;
            let mut high = events.len() - 1;
            while low < high {
                let mid = (low + high) / 2;

                if events[mid].start > events[index].end {
                    high = mid;
                } else {
                    low = mid + 1;
                }
            }
            // tracing::debug!(
            //     "index: {}, low: {}, high: {}, result: {}",
            //     index,
            //     low,
            //     high,
            //     result
            // );
            if events[low].start <= events[index].end {
                continue;
            }
            result = result.max(events[index].value + events[low].suffix_max);
        }
        result = result.max(events[0].suffix_max);

        result
    }
}
struct Solution {}
pub fn run() {
    tracing::info!(
        "{}",
        Solution::max_two_events(vec![vec![1, 3, 2], vec![4, 5, 2], vec![2, 4, 3]])
    );
    tracing::info!(
        "{}",
        Solution::max_two_events(vec![vec![1, 3, 2], vec![4, 5, 2], vec![1, 5, 5]])
    );
    tracing::info!(
        "{}",
        Solution::max_two_events(vec![vec![1, 5, 3], vec![1, 5, 1], vec![6, 6, 5]])
    );
    tracing::info!(
        "{}",
        Solution::max_two_events(vec![
            vec![66, 97, 90],
            vec![98, 98, 68],
            vec![38, 49, 63],
            vec![91, 100, 42],
            vec![92, 100, 22],
            vec![1, 77, 50],
            vec![64, 72, 97]
        ])
    );
}
