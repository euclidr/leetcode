// Definition for a binary tree node.
use std::cell::RefCell;
use std::rc::Rc;

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
    pub fn recover_from_preorder(s: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut pre_d = 0;
        let mut num_start = 0;
        let mut dash_start = 0;
        let mut turn_num = true;

        let mut root = None;
        let mut stack = vec![];

        let s = format!("{}-", s);
        for (idx, c) in s.chars().enumerate() {
            if turn_num && c == '-' {
                turn_num = false;
                dash_start = idx;

                let val: i32 = (&s[num_start..idx]).parse().unwrap();
                let node = Rc::new(RefCell::new(TreeNode::new(val)));
                if pre_d == 0 {
                    stack.push((node.clone(), 0));
                    root = Some(node);
                    continue;
                }

                while let Some((prev, level)) = stack.last() {
                    if level >= &pre_d {
                        stack.pop();
                    } else {
                        let left = prev.borrow().left.clone();
                        match left {
                            Some(_) => prev.borrow_mut().right = Some(node.clone()),
                            None => prev.borrow_mut().left = Some(node.clone()),
                        }
                        break;
                    }
                }

                stack.push((node, pre_d));

                continue;
            }

            if !turn_num && c != '-' {
                turn_num = true;
                num_start = idx;
                pre_d = idx - dash_start;
            }
        }

        root
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::recover_from_preorder("1-2--3--4-5--6--7".to_string())
    );
}
