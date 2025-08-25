impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let mut current = (0, 0);
        let mut result: Vec<i32> = Vec::new();
        let mut upper_right = true;
        while result.len() < mat.len() * mat[0].len() {
            result.push(mat[current.0][current.1]);
            if upper_right {
                if current.1 == mat[0].len() - 1 {
                    current.0 += 1;
                    upper_right = false;
                } else if current.0 == 0 {
                    current.1 += 1;
                    upper_right = false;
                } else {
                    current.0 -= 1;
                    current.1 += 1;
                }
            } else {
                if current.0 == mat.len() - 1 {
                    current.1 += 1;
                    upper_right = true;
                } else if current.1 == 0 {
                    current.0 += 1;
                    upper_right = true;
                } else {
                    current.0 += 1;
                    current.1 -= 1;
                }
            }
        }
        result
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{:?}",
        Solution::find_diagonal_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
    );
}
