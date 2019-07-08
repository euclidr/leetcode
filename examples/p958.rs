use std::rc::Rc;
use std::cell::RefCell;

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
      right: None
    }
  }
}

struct Solution;

impl Solution {
    pub fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }

        let root = root.unwrap();
        let mut node = root.clone();
        let mut height = 0;
        loop {
            let tmp = node;
            match tmp.borrow().left {
                Some(ref left) => {
                    node = left.clone();
                    height = height + 1;
                },
                None => break,
            };
        };

        let mut stack = vec![(root, 0)];
        let mut finished = false;
        while !stack.is_empty() {
            let (node, level) = stack.pop().unwrap();

            if level == height {
                if node.borrow().left.is_some() || node.borrow().right.is_some() {
                    return false
                }
                continue;
            }

            if level < height - 1 {
                if node.borrow().left.is_none() || node.borrow().right.is_none() {
                    return false;
                } else {
                    if let Some(ref right) = node.borrow().right {
                        stack.push((right.clone(), level+1));
                    };
                    if let Some(ref left) = node.borrow().left {
                        stack.push((left.clone(), level+1));
                    };
                }
                continue;
            }

            if level == height - 1 {
                if finished && (node.borrow().left.is_some() || node.borrow().right.is_some()) {
                    return false;
                }
                if node.borrow().left.is_none() && node.borrow().right.is_some() {
                    return false;
                }
                if node.borrow().left.is_some() {
                    if node.borrow().right.is_none() {
                        finished = true;
                    } else {
                        if let Some(ref right) = node.borrow().right {
                            stack.push((right.clone(), level+1));
                        };
                    }
                    if let Some(ref left) = node.borrow().left {
                        stack.push((left.clone(), level+1));
                    };
                    continue;
                }
                finished = true;
            }
        }
        true
    }
}

fn main() {
    Solution::is_complete_tree(None);
}