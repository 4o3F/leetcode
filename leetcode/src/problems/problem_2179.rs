struct FenwickTree {
    tree: Vec<i32>,
}

impl FenwickTree {
    fn new(size: usize) -> Self {
        let n = 1 << (1 + Self::msb(size));
        Self {
            tree: vec![0; n + 1],
        }
    }

    fn add(&mut self, mut index: usize, value: i32) {
        index += 1;
        while index < self.tree.len() {
            self.tree[index] += value;
            index += index & (!index + 1);
        }
    }

    fn query(&self, mut index: usize) -> i32 {
        let mut sum = 0;
        index += 1;
        while index > 0 {
            sum += self.tree[index];
            index &= index - 1;
        }
        sum
    }

    fn msb(mut n: usize) -> usize {
        let mut pos = 0usize;
        while n > 0 {
            n >>= 1;
            pos += 1;
        }
        pos.saturating_sub(1)
    }
}

impl Solution {
    pub fn good_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let n = nums1.len();
        let mut pos2 = vec![0; n];
        for (idx, value) in nums2.into_iter().enumerate() {
            pos2[value as usize] = idx;
        }

        let mut sum_tree = FenwickTree::new(n);

        nums1
            .into_iter()
            .enumerate()
            .map(|(idx, value)| {
                let j = pos2[value as usize];
                let left = sum_tree.query(j as usize) as usize;
                let right = n - j - 1 - (idx - left);
                sum_tree.add(j, 1);
                (left * right) as i64
            })
            .sum()
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{}",
        Solution::good_triplets(vec![2, 0, 1, 3], vec![0, 1, 2, 3])
    );
}
