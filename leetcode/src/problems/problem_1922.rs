impl Solution {
    fn fast_pow(base: i64, pow: i64, modulo: i64) -> i64 {
        let mut base = base % modulo;
        let mut pow = pow;
        let mut result = 1;
        while pow > 0 {
            if pow & 1 == 1 {
                result = result * base % modulo;
            }
            base = base * base % modulo;
            pow = pow >> 1;
        }
        result
    }

    pub fn count_good_numbers(n: i64) -> i32 {
        const MODULO: i64 = 1_000_000_007;
        let even_count = if n % 2 == 0 { n / 2 } else { n / 2 + 1 };
        let odd_count = n / 2;
        // tracing::info!("odd {} even {}", odd_count, even_count);
        // tracing::info!(
        //     "odd count {} even count {}",
        //     Self::fast_pow(4, odd_count, MODULO),
        //     Self::fast_pow(5, even_count, MODULO)
        // );
        (Self::fast_pow(5, even_count, MODULO) * Self::fast_pow(4, odd_count, MODULO) % MODULO)
            as i32
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::count_good_numbers(4));
    tracing::info!("{}", Solution::count_good_numbers(50));
}
