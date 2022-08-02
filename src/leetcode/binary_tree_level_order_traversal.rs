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
    fn solve(arr: &mut Vec<Vec<i32>>, node: Option<&Rc<RefCell<TreeNode>>>, level: usize) {
        if let Some(node) = node {
            let node = Rc::clone(node);
            let node = node.borrow();

            if arr.len() < level + 1 {
                arr.push(Vec::new());
            }
            arr[level].push(node.val);

            Self::solve(arr, node.left.as_ref(), level + 1);
            Self::solve(arr, node.right.as_ref(), level + 1);
        }
    }

    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut arr = Vec::new();
        Self::solve(&mut arr, root.as_ref(), 0);
        arr
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

    let result = Solution::level_order(root);
    println!("{:?}", result);
}
