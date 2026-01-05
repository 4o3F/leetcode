// TODO: redo this problem

use std::collections::{BTreeMap, BTreeSet};

struct AllOne {
    map: BTreeMap<String, i32>,
    set: BTreeSet<(i32, String)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl AllOne {
    fn new() -> Self {
        Self {
            map: BTreeMap::new(),
            set: BTreeSet::new(),
        }
    }

    fn inc(&mut self, key: String) {
        if self.map.contains_key(&key) {
            self.set
                .remove(&(*(self.map.get(&key).unwrap()), key.clone()));
        }

        *self.map.entry(key.clone()).or_insert(0) += 1;

        self.set
            .insert((*(self.map.get(&key).unwrap()), key.clone()));
    }

    fn dec(&mut self, key: String) {
        self.set
            .remove(&(*(self.map.get(&key).unwrap()), key.clone()));

        *self.map.entry(key.clone()).or_insert(0) -= 1;

        if *self.map.get(&key).unwrap() > 0 {
            self.set
                .insert(((*self.map.get(&key).unwrap()), key.clone()));
        }
    }

    fn get_max_key(&self) -> String {
        if let Some(ret) = self.set.iter().next_back() {
            ret.1.clone()
        } else {
            "".to_string()
        }
    }

    fn get_min_key(&self) -> String {
        if let Some(ret) = self.set.iter().next() {
            ret.1.clone()
        } else {
            "".to_string()
        }
    }
}
fn main() {
    use utils::prelude::*;
    init_logger();
    let mut obj = AllOne::new();
    obj.inc("hello".to_string());
    obj.inc("hello".to_string());
    obj.inc("hello".to_string());
    obj.dec("hello".to_string());
    obj.dec("hello".to_string());
    tracing::info!("{}", obj.get_max_key());
    tracing::info!("{}", obj.get_min_key());
}
