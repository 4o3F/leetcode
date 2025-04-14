use std::collections::HashSet;

impl Solution {
    pub fn count_good_integers(n: i32, k: i32) -> i64 {
        let mut ans: i64 = 0;
        // each element is a digits discription of a legal k-palindrome
        // (how many 0,1,2,3,... does it have)
        let mut set: HashSet<Vec<usize>> = HashSet::new();

        // get all k-palindromes
        let k_plds = Self::get_k_pld(n, k);

        // rearrange each k-palindrome, check if it's already in set
        for k_pld in k_plds.into_iter() {
            let v = Self::num2vec(k_pld);
            if set.contains(&v) {
                continue;
            }
            set.insert(v.clone());
            ans += Self::rearrange(&v);
        }

        ans
    }

    // to get all possible k-palindromes
    // we simply get all possible palindromes 
    // and filter those are divisible by k
    fn get_k_pld(n: i32, k: i32) -> Vec<i64> {
        Self::get_all_pld(n, true).into_iter()
            .filter(|e| e % k as i64 == 0)
            .collect()
    }

    // we get all possible palindromes in a recursive manner
    // also use a isFirst flag to avoid leading zero
    fn get_all_pld(n: i32, is_first: bool) -> Vec<i64> {
        let start = if is_first { 1 } else { 0 };
        if n == 1 {
            (start..=9).collect()
        } else if n == 2 {
            (start..=9).map(|e| e * 11).collect()
        } else {
            let sub_plds = Self::get_all_pld(n - 2, false);
            let mut all_plds: Vec<i64> = Vec::new();
            for digit in start..=9 {
                for sub_pld in sub_plds.iter() {
                    all_plds.push(sub_pld * 10 + digit + digit * 10i64.pow(n as u32 - 1));
                }
            }
            all_plds
        }
    }

    // this helper function is to convert a k-palindrome 
    // into its Vec<usize> representation
    // the n-th value of Vec<usize> stands for 
    // how many digits in this k-palindrome are equal to n
    // for example num2vec(23440) -> vec![1,0,1,1,2,0,0,0,0,0]
    fn num2vec(mut num: i64) -> Vec<usize> {
        let mut v = vec![0; 10];
        while num > 0 {
            v[(num % 10) as usize] += 1;
            num /= 10;
        }
        v
    }

    // here we count all valid permutations of a k-palindrome
    // check the Count Permutations section at the bottom
    // to know the mathematics
    fn rearrange(v: &Vec<usize>) -> i64 {
        let non_zero = v.iter().skip(1).sum::<usize>();
        let mut cnt = non_zero as i64 * Self::factorial((non_zero + v[0] - 1) as i64);
        for &n in v.iter() {
            cnt /= Self::factorial(n as i64);
        }
        cnt
    }

    fn factorial(n: i64) -> i64 {
        (1..=n).product::<i64>()
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::count_good_integers(3, 5));
}