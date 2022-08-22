use crate::leetcode::modules::tree_node::TreeNode;
struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn solve(
        subtree: Rc<RefCell<TreeNode>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> (bool, Option<Rc<RefCell<TreeNode>>>) {
        let subtree = Rc::clone(&subtree);
        let tree = subtree.borrow();
        let p = p.unwrap();
        let q = q.unwrap();

        let mut result: Option<Rc<RefCell<TreeNode>>> = None;

        let mut left_has = false;
        if let Some(left) = tree.left.as_ref() {
            let p = Some(Rc::clone(&p));
            let q = Some(Rc::clone(&q));
            let left = Rc::clone(left);
            let (has_p_or_q, res) = Solution::solve(left, p, q);
            left_has = has_p_or_q;
            result = if res.is_some() { res } else { result };
        }

        let mut right_has = false;
        if let Some(right) = tree.right.as_ref() {
            let p = Some(Rc::clone(&p));
            let q = Some(Rc::clone(&q));
            let right = Rc::clone(right);
            let (has_p_or_q, res) = Solution::solve(right, p, q);
            right_has = has_p_or_q;
            result = if res.is_some() { res } else { result };
        }

        let p = p.borrow();
        let q = q.borrow();
        let root_has = tree.val == p.val || tree.val == q.val;

        if (left_has && right_has) || (root_has && left_has) || (root_has && right_has) {
            let tree = Rc::clone(&subtree);
            result = Some(tree);
        }

        (root_has || left_has || right_has, result)
    }

    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let root = root.unwrap();
        let subtree = Rc::clone(&root);
        Solution::solve(subtree, p, q).1
    }
}

pub fn run() {
    let root = TreeNode::try_from("[3,5,1,6,2,0,8,null,null,7,4]").unwrap();
    let root = Some(Rc::new(RefCell::new(root)));
    let p = 5;
    let p = Some(Rc::new(RefCell::new(TreeNode::new(p))));
    let q = 4;
    let q = Some(Rc::new(RefCell::new(TreeNode::new(q))));

    let res = Solution::lowest_common_ancestor(root, p, q).unwrap();
    let res = res.borrow();
    println!("{}", res);
}
