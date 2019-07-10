struct Solution;
impl Solution {
    pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 || nums.len() % 2 == 1 {
            return 0
        }

        let mut nums = nums;
        nums.sort();
        
        let mut idx = 0;
        let mut result = 0;
        let len = nums.len();
        while idx < len {
            result += nums[idx];
            idx += 2;
        }
        result
    }
}

fn main() {
    println!("{}", Solution::array_pair_sum(vec![1,4,3,2]));
}