struct Solution;
impl Solution {
    pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
        let mut words = words;
        for word in words.iter_mut() {
            *word = word.chars().rev().collect::<String>();
        }
        words.sort();

        let mut pre = &words[0];
        let mut cnt = 1;
        let mut sum = words[0].len();
        for idx in 1..words.len() {
            if words[idx].len() < pre.len() {
                cnt += 1;
                pre = &words[idx];
                sum += pre.len();
                continue;
            }

            if pre != &words[idx][..pre.len()] {
                cnt += 1;
                sum += words[idx].len();
            } else {
                sum = sum - pre.len() + words[idx].len();
            }
            pre = &words[idx];
        }

        (sum + cnt) as i32
    }
}

fn main() {
    let result = Solution::minimum_length_encoding(vec![
        "time".to_string(),
        "me".to_string(),
        "bell".to_string(),
    ]);
    println!("{}", result);
}
