struct Version {
    version: Vec<u32>,
}

impl Version {
    fn new(version: String) -> Self {
        let version = version
            .split(".")
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        Self { version }
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let max_idx = self.version.len().max(other.version.len());
        for idx in 0..max_idx {
            let v1 = self.version.get(idx).unwrap_or(&0);
            let v2 = other.version.get(idx).unwrap_or(&0);
            if v1 != v2 {
                return v1.cmp(v2);
            }
        }
        std::cmp::Ordering::Equal
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Version {}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version
    }
}

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let version1 = Version::new(version1);
        let version2 = Version::new(version2);
        match version1.cmp(&version2) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        }
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::compare_version("1.2".to_string(), "1.10".to_string())
    )
}
