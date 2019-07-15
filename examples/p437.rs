use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        let mut stack = vec![];
        let mut result = 0;
        match root {
            Some(n) => stack.push((n.clone(), vec![n.borrow().val])),
            None => return 0,
        }

        while let Some((n, mut vals)) = stack.pop() {
            for v in vals.iter() {
                if *v == sum {
                    result += 1;
                }
            }

            if let Some(ref left) = n.borrow().left {
                let mut vals_left = vals.clone();
                for v in vals_left.iter_mut() {
                    *v = *v + left.borrow().val;
                }
                vals_left.push(left.borrow().val);
                stack.push((left.clone(), vals_left));
            }

            if let Some(ref right) = n.borrow().right {
                for v in vals.iter_mut() {
                    *v = *v + right.borrow().val;
                }
                vals.push(right.borrow().val);
                stack.push((right.clone(), vals));
            }
        }

        result
    }
}

fn main() {}
