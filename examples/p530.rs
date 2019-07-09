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
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let None = root {
            return 0;
        }

        let mut ordered = vec![];
        let mut stack = vec![];

        stack.push((root.unwrap(), false));
        while let Some(item) = stack.pop() {
            if item.1 {
                ordered.push(item.0.borrow().val);
                continue;
            }

            if let Some(ref right) = item.0.borrow().right {
                stack.push((right.clone(), false));
            }

            stack.push((item.0.clone(), true));

            if let Some(ref left) = item.0.borrow().left {
                stack.push((left.clone(), false));
            }
        }

        if ordered.len() == 1 {
            return 0;
        }

        let mut result = ordered[1] - ordered[0];
        for i in 2..ordered.len() {
            if ordered[i] - ordered[i - 1] < result {
                result = ordered[i] - ordered[i - 1]
            }
        }

        result
    }
}

fn main() {
    let tree = Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            right: None,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
        }))),
    }));

    println!("{}", Solution::get_minimum_difference(Some(tree)));
}
