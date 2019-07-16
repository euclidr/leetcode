struct Solution;

impl Solution {
    pub fn flipgame(fronts: Vec<i32>, mut backs: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        for i in 0..fronts.len() {
            if fronts[i] == backs[i] {
                set.insert(fronts[i]);
            }
        }
        let mut nums = fronts;
        nums.append(&mut backs);
        nums.sort();
        nums.dedup();

        for n in nums.iter() {
            if !set.contains(n) {
                return *n;
            }
        }

        0
    }
}

fn main() {
    println!("{}", Solution::flipgame(vec![1,2,4,4,7], vec![1,3,4,1,3]));
}