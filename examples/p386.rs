struct Solution;

impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut result = vec![];
        for i in 1..10 {
            if i > n {
                break
            }
            result.push(i);
            Solution::lexical_order2(&mut result, i*10, n);
        }
        result
    }

    pub fn lexical_order2(result: &mut Vec<i32>, base: i32, target: i32) {
        for i in 0..10 {
            let x = base + i;
            if x > target {
                break
            }
            result.push(x);
            Solution::lexical_order2(result, x*10, target)
        }
    }
}

fn main() {
    let result = Solution::lexical_order(137);
    println!("{:?}", result);
}