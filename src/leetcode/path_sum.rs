struct Solution;

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
    // to be honest, want to add this to the TreeNode impl.
    fn is_leaf(node: &Rc<RefCell<TreeNode>>) -> bool {
        let node = Rc::clone(&node);
        let node = node.borrow();
        node.left.is_none() && node.right.is_none()
    }

    fn solve(node: Option<&Rc<RefCell<TreeNode>>>, target_sum: i32, current_sum: i32) -> bool {
        if let Some(node) = node {
            let sum = node.borrow().val + current_sum;
            if Self::is_leaf(node) {
                sum == target_sum
            } else {
                Self::solve(node.borrow().left.as_ref(), target_sum, sum)
                    || Self::solve(node.borrow().left.as_ref(), target_sum, sum)
            }
        } else {
            false
        }
    }
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if root.is_none() {
            return false;
        }
        Self::solve(root.as_ref(), target_sum, 0)
    }
}

pub fn run() {
    let root = Rc::new(RefCell::new(TreeNode::new(1)));
    root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    if let Some(node) = root.borrow_mut().left.as_ref() {
        node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    }
    if let Some(node) = root.borrow_mut().right.as_ref() {
        node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    }
    let root = Some(root);
    let target_sum = 9;

    let result = Solution::has_path_sum(root, target_sum);
    println!("{}", result);
}
