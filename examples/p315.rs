struct Solution;

impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;

        let mut elements = nums.clone();
        elements.sort();
        elements.dedup();

        let mut indexer = HashMap::new();
        for (idx, val) in elements.iter().enumerate() {
            indexer.insert(*val, idx);
        }

        let mut bitree = vec![0; elements.len()+1];
        let mut result = vec![];

        for n in nums.iter().rev() {
            let pos = indexer[n] + 1;
            Solution::inc(&mut bitree, pos);
            result.push(Solution::query(&mut bitree, pos - 1));
        }

        result.reverse();
        result
    }

    fn inc(bitree: &mut Vec<i32>, mut idx: usize) {
        let len = bitree.len();
        while idx < len {
            bitree[idx] += 1;
            idx += Solution::lowbit(idx);
        }
    }

    fn query(bitree: &mut Vec<i32>, mut idx: usize) -> i32 {
        let mut result = 0;
        while idx > 0 {
            result += bitree[idx];
            idx -= Solution::lowbit(idx);
        }
        result
    }

    fn lowbit(idx: usize) -> usize {
        let idx = idx as isize;
        ( idx & (-idx) ) as usize
    }
}

fn main() {
    println!("{:?}", Solution::count_smaller(vec![5,2,6,1]));
}