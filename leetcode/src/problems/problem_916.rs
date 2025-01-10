impl Solution {
    pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        let mut max_freq = vec![0; 26];
        for word in &words2 {
            let mut wordfreq = vec![0; 26];
            word.chars().for_each(|c| wordfreq[c as usize - 'a' as usize] += 1);
            for i in 0..26 {
                max_freq[i] = max_freq[i].max(wordfreq[i]);
            }
        }
        // tracing::info!("{:?}", max_freq);
        let mut result = Vec::new();
        for word in words1 {
            let mut wordfreq = vec![0; 26];
            word.chars().for_each(|c| wordfreq[c as usize - 'a' as usize] += 1);
            // tracing::info!("{} {:?}", word, wordfreq);
            let mut flag = true;
            for i in 0..26 {
                if wordfreq[i] < max_freq[i] {
                    flag = false;
                    break;
                }
            }
            if flag {
                result.push(word);
            }
        }
        result
    }
}

struct Solution;

pub fn run() {
    let words1 = vec!["amazon","apple","facebook","google","leetcode"].into_iter().map(|x| x.to_string()).collect::<Vec<_>>();
    let words2 = vec!["lo","eo"].into_iter().map(|x| x.to_string()).collect::<Vec<_>>();
    tracing::info!("{:?}", Solution::word_subsets(words1, words2))
}