// Definition for a binary tree node.
use std::rc::Rc;
use std::cell::RefCell;

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
      right: None
    }
  }
}

struct Solution;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        if let None = root {
            return result;
        }

        let mut stack = vec![(root.unwrap(), 1)];
        while !stack.is_empty() {
            let item = stack.pop().unwrap();
            let node = item.0.clone();
            if item.1 > result.len() {
                result.push(node.borrow().val);
            }

            if let Some(ref left) = node.borrow().left {
                stack.push((left.clone(), item.1+1));
            };
            if let Some(ref right) = node.borrow().right {
                stack.push((right.clone(), item.1+1));
            };
        }

        result
    }
}

fn main() {
    Solution::right_side_view(None);
}