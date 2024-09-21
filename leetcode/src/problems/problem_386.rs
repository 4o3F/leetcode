impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::with_capacity(n as usize);
        let mut current = 1;
        for _ in 0..n {
            result.push(current);
            if current * 10 <= n {
                current = current * 10;
            } else {
                while current % 10 == 9 || current + 1 > n {
                    current = current / 10;
                }
                current += 1;
            }
        }

        result
    }
}
struct Solution {}
pub fn run() {
    println!("{:?}", Solution::lexical_order(13));
}
