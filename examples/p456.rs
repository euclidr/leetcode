struct Solution;

impl Solution {
    pub fn find132pattern(mut nums: Vec<i32>) -> bool {
        use std::i32;
        if nums.len() < 3 {
            return false;
        }
        nums.reverse();

        let mut stack = vec![];
        let mut throwed = i32::MIN;
        for i in 0..nums.len() {
            while stack.len() > 0 && stack[stack.len()-1] < nums[i] {
                if let Some(top) = stack.pop() {
                    if top < throwed {
                        return true
                    }
                    throwed = top;
                }
            }
            stack.push(nums[i]);
        }

        if let Some(top) = stack.pop() {
            if top < throwed {
                return true;
            }
        }
        false
    }
}


fn main() {
    println!("{}", Solution::find132pattern(vec![1,2,3,4]));
    println!("{}", Solution::find132pattern(vec![2,1,2,3,4]));
    println!("{}", Solution::find132pattern(vec![4,3,2,1]));
    println!("{}", Solution::find132pattern(vec![3,4,3,2,1]));
    println!("{}", Solution::find132pattern(vec![3,1,2,4,2,4]));
    println!("{}", Solution::find132pattern(vec![-1,3,2,0]));
    println!("{}", Solution::find132pattern(vec![1,0,-4,-3]));
}

// slow
// impl Solution {
//     pub fn find132pattern(mut nums: Vec<i32>) -> bool {
//         if nums.len() < 3 {
//             return false;
//         }

//         nums.dedup();
//         let mut peeks = vec![];
//         let mut mins = vec![0; nums.len()];
//         let mut min = nums[0];
//         for i in 0..nums.len() {
//             if nums[i] < min {
//                 min = nums[i];
//             }
//             mins[i] = min;
//         }

//         for i in 1..(nums.len()-1) {
//             if nums[i] > nums[i+1] && nums[i] > nums[i-1] {
//                 if mins[i-1] < nums[i+1] {
//                     return true;
//                 }

//                 peeks.push((i, nums[i]));
//             }
//         }

//         peeks.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

//         for &(idx, peek) in peeks.iter() {
//             if peek <= mins[idx-1]+1 {
//                 continue;
//             }

//             for idx2 in (idx+1)..nums.len() {
//                 if nums[idx2] < peek && nums[idx2] > mins[idx-1] {
//                     return true
//                 }
//             }

//         }

//         false
//     }
// }