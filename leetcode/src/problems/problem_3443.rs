impl Solution {
    pub fn max_distance(input: String, k: i32) -> i32 {
        let (mut w, mut e, mut s, mut n) = (0, 0, 0, 0);
        let mut max = 0;
        for c in input.chars() {
            match c {
                'S' => s += 1,
                'N' => n += 1,
                'W' => w += 1,
                'E' => e += 1,
                _ => unreachable!(),
            }
            max = max.max(s + e - w - n + 2 * k.min(w + n));
            max = max.max(s + w - e - n + 2 * k.min(e + n));
            max = max.max(n + e - w - s + 2 * k.min(w + s));
            max = max.max(n + w - e - s + 2 * k.min(e + s));
        }
        max
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::max_distance("NWSE".to_string(), 1))
}
