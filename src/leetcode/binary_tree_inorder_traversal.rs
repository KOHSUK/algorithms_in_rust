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
    fn solve(arr: &mut Vec<i32>, node: Option<&Rc<RefCell<TreeNode>>>) {
        if let Some(node) = node {
            let node = Rc::clone(node);
            let node = node.borrow();

            // if this is a leaf node, then push its value into the result vector.
            if node.left.is_none() && node.right.is_none() {
                arr.push(node.val);
                return;
            }
            Self::solve(arr, node.left.as_ref());
            arr.push(node.val);
            Self::solve(arr, node.right.as_ref());
        }
    }

    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut arr = Vec::new();
        Self::solve(&mut arr, root.as_ref());
        arr
    }
}

pub fn run() {
    let root = Rc::new(RefCell::new(TreeNode::new(1)));
    root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    if let Some(node) = root.borrow_mut().right.as_ref() {
        node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    }
    let root = Some(root);

    let result = Solution::inorder_traversal(root);
    println!("{:?}", result);
}
