struct Solution;

impl Solution {
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        let mut digits = vec![];
        for n in num.chars() {
            digits.push(n as i32 - '0' as i32)
        }

        Solution::add_operators2(&digits, 0, 1, target)
    }

    fn add_operators2(digits: &Vec<i32>, start: usize, pre: i32, target: i32) -> Vec<String> {
        let mut result = vec![];
        for gap in (start+1)..(digits.len()+1) {
            let mut first: i32 = 0;

            for i in start..gap {
                first = match first.checked_mul(10) {
                    // Some(m) => m + digits[i],
                    Some(m) => match m.checked_add(digits[i]) {
                        Some(m) => m,
                        None => return result,
                    },
                    None => return result,
                };
            }

            if gap-start > 1 && digits[start] == 0 {
                break;
            }

            let base = match first.checked_mul(pre) {
                Some(m) => m,
                None => return result,
            };

            if gap == digits.len() {
                if base == target {
                    result.push(first.to_string())
                }
                break;

            }

            // plus
            let parts = Solution::add_operators2(digits, gap, 1, target-base);
            for part in parts {
                result.push(format!("{}+{}", first, part))
            }
            
            // minus
            let parts = Solution::add_operators2(digits, gap, -1, target-base);
            for part in parts {
                result.push(format!("{}-{}", first, part))
            }

            // multiply
            let parts = Solution::add_operators2(digits, gap, base, target);
            for part in parts {
                result.push(format!("{}*{}", first, part))
            }

        }
        result
    }

}

fn main() {
    // let result = Solution::add_operators("232".to_string(), 8);
    // println!("{:?}", result);
    // let result = Solution::add_operators("105".to_string(), 5);
    // println!("{:?}", result);
    // let result = Solution::add_operators("3456237490".to_string(), 9191);
    // println!("{:?}", result);
    let result = Solution::add_operators("2147483648".to_string(), -2147483648);
    println!("{:?}", result);
}
