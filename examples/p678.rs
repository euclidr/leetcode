struct Solution;

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        fn check_valid(chars: &Vec<char>, table: &mut Vec<Vec<Option<bool>>>, start: usize, end: usize) -> bool {
            if start > end {
                return true;
            }

            if start == end {
                if chars[start] == '*' {
                    return true
                }
                return false
            }

            if let Some(r) = table[start][end] {
                return r;
            }

            if chars[start] == ')' || chars[end] == '(' {
                table[start][end] = Some(false);
                return false;
            }

            if check_valid(chars, table, start+1, end-1) {
                table[start][end] = Some(true);
                return true;
            }

            let mut idx = start;
            while idx < end {
                if check_valid(chars, table, start, idx) && check_valid(chars, table, idx+1, end) {
                    table[start][end] = Some(true);
                    return true;
                }
                idx += 1;
            }

            table[start][end] = Some(false);
            false
        }

        if s.len() == 0 {
            return true;
        }

        let mut table = vec![vec![None; s.len()]; s.len()];
        check_valid(&s.chars().collect(), &mut table, 0, s.len()-1)
    }
}

fn main() {
    println!("{:?}", Solution::check_valid_string("(*))".to_string()));
    println!("{:?}", Solution::check_valid_string("(*)))".to_string()));
}