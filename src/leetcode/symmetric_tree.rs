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
    fn stack_sub_tree(node: Option<&Rc<RefCell<TreeNode>>>, stack: &mut Vec<i32>) {
        if let Some(node) = node {
            let node = Rc::clone(node);
            let node = node.borrow();

            Self::stack_sub_tree(node.left.as_ref(), stack);
            Self::stack_sub_tree(node.right.as_ref(), stack);
            stack.push(node.val);
        } else {
            stack.push(9999);
        }
    }

    fn check_sub_tree(node: Option<&Rc<RefCell<TreeNode>>>, stack: &mut Vec<i32>) -> bool {
        if let Some(node) = node {
            let node = Rc::clone(node);
            let node = node.borrow();
            let res = if let Some(test) = stack.pop() {
                test == node.val
            } else {
                false
            };
            res && Self::check_sub_tree(node.left.as_ref(), stack)
                && Self::check_sub_tree(node.right.as_ref(), stack)
        } else {
            let res = stack.pop().unwrap();
            res == 9999
        }
    }

    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(root) = root {
            let mut stack = Vec::new();
            Self::stack_sub_tree(root.borrow().left.as_ref(), &mut stack);
            Self::check_sub_tree(root.borrow().right.as_ref(), &mut stack)
        } else {
            false
        }
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

    let result = Solution::is_symmetric(root);
    println!("{}", result);
}
