impl Solution {
    pub fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
        let mut folder = folder;
        folder.sort();

        let mut result: Vec<String> = Vec::new();
        result.push(folder[0].clone());
        for i in 1..folder.len() {
            let mut last = result.last().unwrap().to_owned();
            last.push('/');
            if !folder[i].starts_with(last.as_str()) {
                result.push(folder[i].clone());
            }
        }
        return result;
    }
}

struct Solution {}
pub fn run() {
    let arr = vec!["/a/b/c", "/a/b/ca", "/a/b/d"];
    let arr = arr.into_iter().map(|x| x.to_string()).collect::<Vec<_>>();
    tracing::info!("{:?}", Solution::remove_subfolders(arr));
}
