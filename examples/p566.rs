struct Solution;

impl Solution {
    pub fn matrix_reshape(nums: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let (r, c) = (r as usize, c as usize);
        let rows = nums.len();
        if rows == 0 {
            return nums;
        }
        let cols = nums[0].len();
        if rows*cols != r*c {
            return nums;
        }

        let mut result = vec![vec![0; c]; r];

        for row in 0..rows {
            for col in 0..cols {
                let r1 = (row*cols + col) / c;
                let c1 = (row*cols + col) % c;
                result[r1][c1] = nums[row][col];
            }
        }

        result
    }
}

fn main() {
    let result = Solution::matrix_reshape(vec![vec![1,2], vec![3,4]], 1, 4);
    println!("{:?}", result);
}