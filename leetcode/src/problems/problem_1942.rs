use std::{cmp::Reverse, collections::BinaryHeap, i32};

impl Solution {
    pub fn smallest_chair(times: Vec<Vec<i32>>, target_friend: i32) -> i32 {
        // (friend_index, time)
        let mut friend_time_map: Vec<(i32, Vec<i32>)> = times
            .into_iter()
            .enumerate()
            .map(|(index, time)| (index as i32, time))
            .collect();

        // Sort by enter time
        friend_time_map.sort_unstable_by_key(|(_, time)| time[0]);

        tracing::trace!("Sorted friend_time_map: {:#?}", friend_time_map);
        // (leave_time, chair_index)
        let mut leaves = BinaryHeap::<Reverse<(i32, i32)>>::new();
        // available chair index
        let mut available = BinaryHeap::<Reverse<i32>>::new();

        let mut min = 0..i32::MAX;
        for (friend_index, time) in friend_time_map.iter() {
            tracing::trace!("Current available chair: {:#?}", available);
            tracing::trace!("Current leave time: {:#?}", leaves);
            tracing::trace!("Current time: {:#?}", time);
            while let Some(Reverse((leave_time, chair_index))) = leaves.peek() {
                // If the chair is empty before current enter
                if leave_time <= &time[0] {
                    tracing::trace!("Leave chair: {:#?}", chair_index);
                    available.push(Reverse(*chair_index));
                    leaves.pop();
                } else {
                    break;
                }
            }

            let current = available.pop().or_else(|| min.next().map(Reverse)).unwrap();
            if friend_index == &target_friend {
                return current.0;
            }

            leaves.push(Reverse((time[1], current.0)));
            tracing::trace!("##########################################");
        }

        unreachable!()
    }
}

struct Solution {}
pub fn run() {
    // let times = vec![vec![1, 4], vec![2, 3], vec![4, 6]];
    // let target_friend = 1;
    // tracing::info!("{:?}", Solution::smallest_chair(times, target_friend));

    // let times = vec![vec![3, 10], vec![1, 5], vec![2, 6]];
    // let target_friend = 0;
    // tracing::info!("{:?}", Solution::smallest_chair(times, target_friend));

    let times = vec![
        vec![33889, 98676],
        vec![80071, 89737],
        vec![44118, 52565],
        vec![52992, 84310],
        vec![78492, 88209],
        vec![21695, 67063],
        vec![84622, 95452],
        vec![98048, 98856],
        vec![98411, 99433],
        vec![55333, 56548],
        vec![65375, 88566],
        vec![55011, 62821],
        vec![48548, 48656],
        vec![87396, 94825],
        vec![55273, 81868],
        vec![75629, 91467],
    ];
    let target_friend = 6;
    tracing::info!("{:?}", Solution::smallest_chair(times, target_friend));
}
