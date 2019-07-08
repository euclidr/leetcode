use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
struct Solution;
impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut counter = HashMap::new();
        for word in words.iter() {
            match counter.get_mut(word.as_str()) {
                Some(n) => {
                    *n = *n + 1;
                }
                None => {
                    counter.insert(word.as_str(), 1);
                }
            }
        }

        #[derive(Eq, PartialEq)]
        struct Item<'a> {
            word: &'a str,
            cnt: i32,
        }

        impl<'a> Ord for Item<'a> {
            fn cmp(&self, other: &Item) -> Ordering {
                other
                    .cnt
                    .cmp(&self.cnt)
                    .then_with(|| self.word.cmp(&other.word))
            }
        }

        impl<'a> PartialOrd for Item<'a> {
            fn partial_cmp(&self, other: &Item) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        let mut heap = BinaryHeap::new();

        for (word, cnt) in counter.into_iter() {
            heap.push(Item { word, cnt });
            if heap.len() > k as usize {
                heap.pop();
            }
        }

        let mut result = vec![];

        while let Some(Item { word, cnt: _ }) = heap.pop() {
            result.push(word.to_string());
        }

        result.reverse();
        result
    }
}

fn main() {
    let result = Solution::top_k_frequent(
        vec![
            "i".to_string(),
            "love".to_string(),
            "leetcode".to_string(),
            "i".to_string(),
            "love".to_string(),
            "coding".to_string(),
        ],
        2,
    );

    println!("{:?}", result);
}
