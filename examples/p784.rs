struct Solution;

impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        let mut result = vec!["".to_string()];
        let dif = ('a' as u8) - ('A' as u8);
        for c in s.chars() {
            let a = c;
            let b;
            if c >= 'a' && c <= 'z' {
                b = ((c as u8) - dif) as char;
            } else if c >= 'A' && c <= 'Z' {
                b = ((c as u8) + dif) as char;
            } else {
                for line in result.iter_mut() {
                    line.push(c)
                }
                continue
            }

            let mut new_result = vec![];
            for line in result.iter() {
                let mut line_a = line.clone();
                line_a.push(a);
                new_result.push(line_a);
                let mut line_b = line.clone();
                line_b.push(b);
                new_result.push(line_b);
            }
            result = new_result;
        }
        result
    }
}

fn main() {
    println!("{:?}", Solution::letter_case_permutation("a2b3c".to_string()));

}