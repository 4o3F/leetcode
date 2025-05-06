impl Solution {
    fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        let mut cnta = [0; 7];
        let mut cntb = [0; 7];
        let mut same = [0; 7];
        for i in 0..tops.len() {
            cnta[tops[i] as usize] += 1;
            cntb[bottoms[i] as usize] += 1;
            if tops[i] == bottoms[i] {
                same[tops[i] as usize] += 1;
            }
        }
        for i in 1..=6 {
            if cnta[i] + cntb[i] - same[i] == tops.len() {
                return (cnta[i].min(cntb[i]) - same[i]) as i32;
            }
        }
        -1
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{}",
        Solution::min_domino_rotations(vec![2, 1, 2, 4, 2, 2], vec![5, 2, 6, 2, 3, 2])
    );
}
