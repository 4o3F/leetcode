use std::{cmp::Reverse, collections::BinaryHeap, fmt::Debug};

struct Pair {
    idx: usize,
    val: i32,
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl Eq for Pair {}
impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.val.cmp(&other.val)
    }
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    pub fn new(len: usize) -> Self {
        let parent = (0..len).collect();
        UnionFind {
            parent,
            size: vec![1; len],
        }
    }

    pub fn size(&mut self, a: usize) -> usize {
        self.size[a]
    }

    pub fn find(&mut self, a: usize) -> usize {
        if self.parent[a] == a {
            a
        } else {
            self.parent[a] = self.find(self.parent[a]);
            self.parent[a]
        }
    }

    pub fn union(&mut self, a: usize, b: usize) {
        let a_parent = self.find(a);
        let b_parent = self.find(b);
        if a_parent == b_parent {
            return;
        }
        if self.size[a_parent] < self.size[b_parent] {
            self.parent[a_parent] = b_parent;
            self.size[b_parent] += self.size[a_parent];
        } else {
            self.parent[b_parent] = a_parent;
            self.size[a_parent] += self.size[b_parent];
        }
    }
}

impl Debug for UnionFind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UnionFind")
            .field("parent", &self.parent)
            .field("size", &self.size)
            .finish()
    }
}

impl Solution {
    pub fn min_capability(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::from_iter(nums.iter().enumerate().map(|p| {
            Reverse(Pair {
                idx: p.0,
                val: *p.1,
            })
        }));
        let mut uf = UnionFind::new(nums.len());
        let mut current = 0i32;
        let mut visited = vec![false; nums.len()];
        let mut change = 0;
        while current < k {
            let pair = heap.pop().unwrap().0;
            change = pair.val;
            visited[pair.idx] = true;
            if pair.idx > 0 && visited[pair.idx - 1] {
                let idx = uf.find(pair.idx - 1);
                current -= uf.size(idx).div_ceil(2) as i32;
                uf.union(pair.idx, pair.idx - 1);
            }

            if pair.idx < nums.len() - 1 && visited[pair.idx + 1] {
                let idx = uf.find(pair.idx + 1);
                current -= uf.size(idx).div_ceil(2) as i32;
                uf.union(pair.idx, pair.idx + 1);
            }

            let idx = uf.find(pair.idx);
            current += uf.size(idx).div_ceil(2) as i32;
            // tracing::info!("{} {:?} {:?}", current, visited, uf);
        }
        change
    }

    pub fn min_capability_binary_search(nums: Vec<i32>, k: i32) -> i32 {
        let mut sorted_unique = nums.clone();
        sorted_unique.sort();

        let index = sorted_unique.partition_point(|&h| !Self::is_valid(&nums, k, h));
        sorted_unique[index]
    }

    fn is_valid(nums: &[i32], k: i32, h: i32) -> bool {
        let (mut prev, mut curr) = (0, 0);
        for &num in nums {
            let take = if num <= h { 1 } else { 0 };
            let new_curr = curr.max(prev + take);
            prev = curr;
            curr = new_curr;
            if curr >= k {
                return true;
            }
        }
        curr >= k
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::min_capability(vec![2, 3, 5, 9], 2));
    tracing::info!("{}", Solution::min_capability(vec![9, 6, 20, 21, 8], 3));
    tracing::info!(
        "{}",
        Solution::min_capability_binary_search(vec![2, 3, 5, 9], 2)
    );
    tracing::info!(
        "{}",
        Solution::min_capability_binary_search(vec![9, 6, 20, 21, 8], 3)
    );
}
