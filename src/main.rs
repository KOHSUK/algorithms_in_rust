mod algorithms;
mod aoj;
mod codeforces;
mod leetcode;

// use algorithms::*;
// use aoj::*;
// use codeforces::gym::*;

use leetcode::modules::tree_node::TreeNode;

fn main() {
    let root = TreeNode::try_from(vec![
        "5", "4", "8", "11", "null", "13", "4", "7", "2", "null", "null", "null", "1",
    ])
    .unwrap();
    println!("{}", root);
}
