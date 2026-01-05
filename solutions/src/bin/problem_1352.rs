struct ProductOfNumbers {
    prefix_product: Vec<i32>,
    last_zero: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ProductOfNumbers {
    fn new() -> Self {
        Self {
            prefix_product: vec![1],
            last_zero: -1,
        }
    }

    fn add(&mut self, num: i32) {
        let n = self.prefix_product.len();
        self.prefix_product
            .push(1.max(num * self.prefix_product.last().unwrap()));
        if num == 0 {
            self.last_zero = n as i32;
        }
    }

    fn get_product(&self, k: i32) -> i32 {
        // tracing::info!("{:?}", self.prefix_product);
        let n = self.prefix_product.len();
        let index = n as i32 - 1 - k;
        if index >= self.last_zero {
            self.prefix_product[n - 1] / self.prefix_product[index as usize]
        } else {
            0
        }
    }
}

fn main() {
    use utils::prelude::*;
    init_logger();
    let mut obj = ProductOfNumbers::new();
    obj.add(3);
    obj.add(0);
    obj.add(2);
    obj.add(5);
    obj.add(4);
    tracing::info!("{}", obj.get_product(2));
    tracing::info!("{}", obj.get_product(3));
    tracing::info!("{}", obj.get_product(4));
    obj.add(8);
    tracing::info!("{}", obj.get_product(2));
}
