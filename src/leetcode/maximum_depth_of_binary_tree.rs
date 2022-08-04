struct Solution;

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

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn solve(node: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = node {
            let node = Rc::clone(node);
            let node = node.borrow();

            let left = Self::solve(node.left.as_ref());
            let right = Self::solve(node.right.as_ref());
            i32::max(left, right) + 1
        } else {
            0
        }
    }

    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::solve(root.as_ref())
    }
}

pub fn run() {
    let root = Rc::new(RefCell::new(TreeNode::new(3)));
    root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(9))));
    root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(20))));
    if let Some(node) = root.borrow_mut().right.as_ref() {
        node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(15))));
        node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
    }
    let root = Some(root);

    let result = Solution::max_depth(root);
    println!("{}", result);
}
