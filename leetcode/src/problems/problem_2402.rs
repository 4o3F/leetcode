use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn most_booked(n: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut counts = vec![0; n];

        // Min heap for busy rooms: (available_time, room_number)
        let mut busy_rooms = BinaryHeap::with_capacity(n);
        // Min heap for available rooms: room_number
        let mut available_rooms: BinaryHeap<Reverse<usize>> = (0..n).map(Reverse).collect();

        // Sort meetings by start time
        meetings.sort_unstable();

        for meeting in meetings {
            let (start, end) = (meeting[0] as i64, meeting[1] as i64);

            // Free up any rooms that are now available
            while let Some(&Reverse((available_time, room))) = busy_rooms.peek() {
                if available_time > start {
                    break;
                }

                available_rooms.push(Reverse(room));
                busy_rooms.pop();
            }

            match available_rooms.pop() {
                Some(Reverse(room)) => {
                    // Room available at start time
                    busy_rooms.push(Reverse((end, room)));
                    counts[room] += 1;
                }
                None => {
                    // Need to delay the meeting
                    let Reverse((available_time, room)) = busy_rooms.pop().unwrap();
                    busy_rooms.push(Reverse((end - start + available_time, room)));
                    counts[room] += 1;
                }
            }
        }

        // Find room with most meetings (using lowest number for ties)
        counts
            .iter()
            .enumerate()
            .max_by(|&(i1, c1), &(i2, c2)| c1.cmp(c2).then(i2.cmp(&i1)))
            .map(|(i, _)| i as i32)
            .unwrap()
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{}",
        Solution::most_booked(2, vec![vec![0, 10], vec![1, 5], vec![2, 7], vec![3, 4]])
    )
}
