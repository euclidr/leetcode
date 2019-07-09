struct Solution;
impl Solution {
    pub fn largest_overlap(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        let length = a.len();
        for sr in 0..length {
            for sc in 0..length {
                if result > (length - sr) * (length - sc) {
                    break;
                }
                let mut tmp_result1 = 0;
                let mut tmp_result2 = 0;
                for r in 0..length - sr {
                    for c in 0..length - sc {
                        if a[r][c] == 1 && b[sr + r][sc + c] == 1 {
                            tmp_result1 += 1;
                        }
                        if b[r][c] == 1 && a[sr + r][sc + c] == 1 {
                            tmp_result2 += 1;
                        }
                    }
                }
                if tmp_result1 > result {
                    result = tmp_result1;
                }
                if tmp_result2 > result {
                    result = tmp_result2;
                }
            }
        }
        result as i32
    }
}

fn main() {
    Solution::largest_overlap(vec![vec![]; 0], vec![vec![]; 0]);
    for i in 0..10 - 7 {
        println!("{}", i);
    }
}
