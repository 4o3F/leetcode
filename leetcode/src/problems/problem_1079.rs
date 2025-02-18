impl Solution {
    fn permutation(aggregate: &mut Vec<usize>) -> i32 {
        let mut num = 0;
        for i in 0..26 {
            if aggregate[i] > 0 {
                aggregate[i] -= 1;
                num += 1;
                num += Self::permutation(aggregate);
                aggregate[i] += 1;
            }
        }
        num
    }

    pub fn num_tile_possibilities(tiles: String) -> i32 {
        let mut aggregate = vec![0usize; 26];
        for &b in tiles.as_bytes() {
            aggregate[b as usize - 'A' as usize] += 1;
        }
        Self::permutation(&mut aggregate)
    }
}

struct Solution;
pub fn run() {
    tracing::info!("{}", Solution::num_tile_possibilities("AAABBC".to_string()));
}