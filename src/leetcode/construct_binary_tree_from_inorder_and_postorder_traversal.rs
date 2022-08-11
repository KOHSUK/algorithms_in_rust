struct Solution;
use super::modules::tree_node::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn find_pivot(arr: &[i32], postorder: &mut Vec<i32>) -> Option<i32> {
        let mut res = None;
        let mut remove_index = postorder.len();
        for (index, &item) in postorder.iter().enumerate().rev() {
            if arr.contains(&item) {
                res = Some(item);
                remove_index = index;
                break;
            }
        }
        if remove_index != postorder.len() {
            postorder.remove(remove_index);
        }

        res
    }

    fn split(sub_tree: Rc<RefCell<TreeNode>>, arr: &mut Vec<i32>, postorder: &mut Vec<i32>) {
        let target = Rc::clone(&sub_tree);
        let mut target = target.borrow_mut();
        let value = target.val;
        let mut split_iter = arr.split(|&x| x == value);
        if let Some(left) = split_iter.next() {
            let left = left;
            if left.len() == 1 {
                let left_node = Rc::new(RefCell::new(TreeNode::new(left[0])));
                target.left = Some(left_node);
            } else if left.len() > 1 {
                if let Some(next_target) = Self::find_pivot(left, postorder) {
                    let next_target = Rc::new(RefCell::new(TreeNode::new(next_target)));
                    target.left = Some(Rc::clone(&next_target));
                    Solution::split(next_target, &mut left.to_vec(), postorder);
                } else {
                    return;
                }
            }
        }

        if let Some(right) = split_iter.next() {
            let right = right;
            if right.len() == 1 {
                let right_node = Rc::new(RefCell::new(TreeNode::new(right[0])));
                target.right = Some(right_node);
            } else if right.len() > 1 {
                if let Some(next_target) = Self::find_pivot(right, postorder) {
                    let next_target = Rc::new(RefCell::new(TreeNode::new(next_target)));
                    target.right = Some(Rc::clone(&next_target));
                    Solution::split(next_target, &mut right.to_vec(), postorder);
                } else {
                    return;
                }
            }
        }
    }

    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let node = Rc::new(RefCell::new(TreeNode::new(postorder[postorder.len() - 1])));
        let mut inorder = inorder;
        let mut postorder = postorder;
        postorder.pop();

        let sub_tree = Rc::clone(&node);
        Self::split(sub_tree, &mut inorder, &mut postorder);

        Some(Rc::clone(&node))
    }
}

pub fn run() {
    let inorder = vec![9, 3, 15, 20, 7];
    let postorder = vec![9, 15, 7, 20, 3];
    // let inorder = vec![1, 2, 3, 4];
    // let postorder = vec![2, 1, 4, 3];
    let result = Solution::build_tree(inorder, postorder).unwrap();
    println!("{}", result.borrow_mut());
}
