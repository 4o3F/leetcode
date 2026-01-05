impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
        let n = baskets.len();
        let mut seg: Vec<i32> = vec![0; 4 * n];

        fn build(baskets: &Vec<i32>, seg: &mut Vec<i32>, idx: usize, l: usize, r: usize) {
            if l == r {
                seg[idx] = baskets[l];
                return;
            }
            let mid = (l + r) / 2;
            build(baskets, seg, 2 * idx + 1, l, mid);
            build(baskets, seg, 2 * idx + 2, mid + 1, r);
            seg[idx] = seg[2 * idx + 1].max(seg[2 * idx + 2]);
        }

        fn place(seg: &mut Vec<i32>, idx: usize, l: usize, r: usize, val: i32) -> bool {
            if seg[idx] < val {
                return false;
            }
            if l == r {
                seg[idx] = -1;
                return true;
            }
            let mid = (l + r) / 2;
            let mut placed = place(seg, 2 * idx + 1, l, mid, val);
            if !placed {
                placed = place(seg, 2 * idx + 2, mid + 1, r, val);
            }
            seg[idx] = seg[2 * idx + 1].max(seg[2 * idx + 2]);
            placed
        }

        build(&baskets, &mut seg, 0, 0, n - 1);

        let mut unplaced = 0;
        for fruit in fruits {
            if seg[0] < fruit || !place(&mut seg, 0, 0, n - 1, fruit) {
                unplaced += 1;
            }
        }
        unplaced
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::num_of_unplaced_fruits(vec![4, 2, 5], vec![3, 5, 4])
    )
}
