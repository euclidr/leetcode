use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        let (nchars, links) = Solution::resolve_pattern(&pattern);
        println!("nchars: {}", nchars);
        println!("links: {:?}", links);

        let mut result = vec![];
        for word in words.iter() {
            if Solution::match_pattern(word, nchars, &links) {
                result.push(word.clone());
            }
        }

        result
    }

    fn match_pattern(word: &String, nchars: usize, links: &Vec<Option<usize>>) -> bool {
        let chars: Vec<char> = word.chars().collect();
        if chars.len() != links.len() {
            return false;
        }

        let mut table = HashSet::new();
        for c in chars.iter() {
            table.insert(*c);
        }

        if nchars != table.len() {
            return false;
        }

        for (i, l) in links.iter().enumerate() {
            if let Some(next) = *l {
                if chars[i] != chars[next] {
                    return false;
                }
            }
        }
        true
    }

    fn resolve_pattern(pattern: &String) -> (usize, Vec<Option<usize>>) {
        let mut table = HashSet::new();
        let mut links = vec![None; pattern.len()];
        let pattern: Vec<char> = pattern.chars().collect();

        for (i, c) in pattern.iter().enumerate() {
            table.insert(*c);
            for j in i + 1..pattern.len() {
                if *c == pattern[j] {
                    links[i] = Some(j);
                    break;
                }
            }
        }

        (table.len(), links)
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::find_and_replace_pattern(
            vec![
                "mee".to_string(),
                "aqq".to_string(),
                "abc".to_string(),
                "ccc".to_string()
            ],
            "abb".to_string()
        )
    );
}
