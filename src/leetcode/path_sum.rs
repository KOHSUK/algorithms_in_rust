struct Solution;
use super::modules::tree_node::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    // to be honest, want to add this to the TreeNode impl.
    fn is_leaf(node: &Rc<RefCell<TreeNode>>) -> bool {
        let node = Rc::clone(node);
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
                    || Self::solve(node.borrow().right.as_ref(), target_sum, sum)
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
    let root = TreeNode::try_from("[5,4,8,11,null,13,4,7,2,null,null,null,1]").unwrap();
    let root = Some(Rc::new(RefCell::new(root)));
    let target_sum = 22;

    let result = Solution::has_path_sum(root, target_sum);
    println!("{}", result);
}
