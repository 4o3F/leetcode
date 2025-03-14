use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

impl Solution {
    pub fn is_zero_array1(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
        let len = nums.len();
        let diff_array = queries
            .iter()
            .fold(vec![0; len + 1], |mut diff_array, query| {
                diff_array[query[0] as usize] -= 1;
                diff_array[query[1] as usize + 1] += 1;
                diff_array
            });

        let mut decrement = 0;
        for (idx, num) in nums.iter().enumerate() {
            decrement += diff_array[idx];
            if num + decrement > 0 {
                return false;
            }
        }
        true
    }
    pub fn is_zero_array2(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
        let len = nums.len();
        let diff_array = queries
            .into_iter()
            .fold(vec![0; len + 1], |mut diff_array, query| {
                diff_array[query[0] as usize] -= 1;
                diff_array[query[1] as usize + 1] += 1;
                diff_array
            });

        let mut decrement = 0;
        for (idx, num) in nums.into_iter().enumerate() {
            decrement += diff_array[idx];
            if num + decrement > 0 {
                return false;
            }
        }
        true
    }
}

struct Solution;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("iter", |b| {
        b.iter(|| {
            Solution::is_zero_array1(
                black_box(vec![4, 3, 2, 1]),
                black_box(vec![vec![1, 3], vec![0, 2]]),
            )
        })
    });
    c.bench_function("into_iter", |b| {
        b.iter(|| {
            Solution::is_zero_array2(
                black_box(vec![4, 3, 2, 1]),
                black_box(vec![vec![1, 3], vec![0, 2]]),
            )
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
