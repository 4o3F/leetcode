impl Solution {
    pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        let mask = (0..maximum_bit - 1).fold(1, |accumulate, _| (accumulate << 1) | 1);
        let mut result = Vec::new();
        let mut xor = None;
        for idx in 0..nums.len() {
            if xor.is_none() {
                xor = Some(nums[idx])
            } else {
                xor = Some(xor.unwrap() ^ nums[idx])
            }
            // tracing::debug!("xor {:?}", xor);
            // tracing::debug!("not {:#012b}", !xor.unwrap());
            // tracing::debug!(
            //     "and {:#012b}",
            //     (0..maximum_bit - 1).fold(1, |accumulate, _| { (accumulate << 1) | 1 })
            // );
            let tmp = !xor.unwrap() & mask;

            // tracing::debug!("tmp {:#012b}", tmp);
            result.push(tmp);
        }
        result.reverse();
        result
    }
}

struct Solution {}
pub fn run() {
    let arr = vec![0, 1, 1, 3];
    tracing::info!("{:?}", Solution::get_maximum_xor(arr, 2));
    let arr = vec![2, 3, 4, 7];
    tracing::info!("{:?}", Solution::get_maximum_xor(arr, 3));
    let arr = vec![0, 1, 2, 2, 5, 7];
    tracing::info!("{:?}", Solution::get_maximum_xor(arr, 3));
}
