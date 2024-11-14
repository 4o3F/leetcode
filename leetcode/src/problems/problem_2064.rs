use std::collections::BinaryHeap;

#[derive(Debug)]
struct Product {
    store_count: i32,
    quantity: i32,
}

impl Product {
    fn max_count(&self) -> i32 {
        (f64::from(self.quantity) / f64::from(self.store_count)).ceil() as i32
    }
}

impl Ord for Product {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Product {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_weight = self.max_count();
        let other_weight = other.max_count();
        self_weight.partial_cmp(&other_weight)
    }
}

impl Eq for Product {}

impl PartialEq for Product {
    fn eq(&self, other: &Self) -> bool {
        self.store_count == other.store_count && self.quantity == other.quantity
    }
}

// impl Solution {
//     pub fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
//         let mut max = *quantities.iter().max().unwrap();
//         let mut heap: BinaryHeap<Product> = BinaryHeap::new();
//         for &quantity in &quantities {
//             heap.push(Product {
//                 store_count: 1,
//                 quantity,
//             });
//         }
//         let mut store_count = quantities.len() as i32;
//         while store_count < n {
//             let mut worst = heap.pop().unwrap();
//             worst.store_count += 1;
//             max = worst.max_count();
//             // tracing::debug!("worst: {:?} max: {}", worst, max);
//             heap.push(worst);
//             store_count += 1;
//         }

//         // tracing::debug!("{:?}", heap);
//         let worst = heap.pop().unwrap();
//         max.max(worst.max_count())
//     }
// }

impl Solution {
    pub fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
        Vec::from_iter(1..100001)
            .partition_point(|x| n < quantities.iter().map(|&q| 1 + (q - 1) / x).sum())
            as i32
            + 1
    }
}

struct Solution {}
pub fn run() {
    // let quantities = vec![11, 6];
    // tracing::info!("{}", Solution::minimized_maximum(6, quantities));
    // let quantities = vec![15, 10, 10];
    // tracing::info!("{}", Solution::minimized_maximum(7, quantities));
    let quantities = vec![25, 11, 29, 6, 24, 4, 29, 18, 6, 13, 25, 30];
    tracing::info!("{}", Solution::minimized_maximum(22, quantities));
}
