struct Solution;

fn main() {
    println!(
        "{}",
        Solution::check_inclusion("abdd".to_string(), "dededdba".to_string())
    );
}

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s2.len() < s1.len() {
            return false;
        }

        let mut pattern = vec![0; 26];
        let mut sample = vec![0; 26];
        let len = s1.len();
        for c in s1.chars() {
            pattern[(c as usize) - ('a' as usize)] += 1;
        }

        let s2: Vec<char> = s2.chars().collect();
        for i in 0..len {
            sample[(s2[i] as usize) - ('a' as usize)] += 1;
        }

        fn is_same(v1: &Vec<i32>, v2: &Vec<i32>) -> bool {
            for (a, b) in v1.iter().zip(v2.iter()) {
                if *a != *b {
                    return false;
                }
            }
            true
        }
        // println!("{:?} \n{:?}", pattern, sample);

        if is_same(&pattern, &sample) {
            return true;
        }

        for i in len..s2.len() {
            sample[s2[i - len] as usize - 'a' as usize] -= 1;
            sample[s2[i] as usize - 'a' as usize] += 1;
            // println!("----");
            // println!("{:?} \n{:?}", pattern, sample);
            if is_same(&pattern, &sample) {
                return true;
            }
        }

        false
    }
}
