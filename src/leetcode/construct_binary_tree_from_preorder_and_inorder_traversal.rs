struct Solution;
use super::modules::tree_node::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn find_pivot(preorder: &mut Vec<i32>, subtree: &Vec<i32>) -> Option<i32> {
        let mut res = None;
        let mut remove_idx = preorder.len();
        for (index, &item) in preorder.iter().enumerate() {
            if subtree.contains(&item) {
                res = Some(item);
                remove_idx = index;
                break;
            }
        }

        if remove_idx != preorder.len() {
            preorder.remove(remove_idx);
        }

        res
    }

    pub fn construct(preorder: &mut Vec<i32>, inorder: &mut Vec<i32>, root: Rc<RefCell<TreeNode>>) {
        let root = Rc::clone(&root);
        let mut root = root.borrow_mut();
        let root_val = root.val;
        let mut inorder_iter = inorder.split(|&x| x == root_val);

        if let Some(left) = inorder_iter.next() {
            if left.len() == 1 {
                let new_node = Rc::new(RefCell::new(TreeNode::new(left[0])));
                root.left = Some(new_node);
            } else if left.len() > 1 {
                if let Some(pivot) = Self::find_pivot(preorder, &left.to_vec()) {
                    let new_node = Rc::new(RefCell::new(TreeNode::new(pivot)));
                    root.left = Some(Rc::clone(&new_node));
                    Self::construct(preorder, &mut left.to_vec(), new_node);
                }
            }
        }

        if let Some(right) = inorder_iter.next() {
            if right.len() == 1 {
                let new_node = Rc::new(RefCell::new(TreeNode::new(right[0])));
                root.right = Some(new_node);
            } else if right.len() > 1 {
                if let Some(pivot) = Self::find_pivot(preorder, &right.to_vec()) {
                    let new_node = Rc::new(RefCell::new(TreeNode::new(pivot)));
                    root.right = Some(Rc::clone(&new_node));
                    Self::construct(preorder, &mut right.to_vec(), new_node);
                }
            }
        }
    }

    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut preorder = preorder;
        let mut inorder = inorder;
        let pivot = Self::find_pivot(&mut preorder, &inorder).unwrap();
        let root = Rc::new(RefCell::new(TreeNode::new(pivot)));
        Self::construct(&mut preorder, &mut inorder, Rc::clone(&root));
        Some(root)
    }
}

pub fn run() {
    let preorder = vec![3, 9, 20, 15, 7];
    let inorder = vec![9, 3, 15, 20, 7];
    let result = Solution::build_tree(preorder, inorder).unwrap();
    println!("{}", result.borrow_mut());
}
