// https://leetcode.com/explore/learn/card/hash-table/183/combination-with-other-algorithms/1112/

use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut hash_set = HashSet::new();
        for num in nums {
            if hash_set.contains(&num) {
                return true;
            }
            hash_set.insert(num);
        }
        false
    }
}

pub fn run() {
    let nums = vec![1, 2, 3, 4, 1, 2, 3, 4];
    let result = Solution::contains_duplicate(nums);
    println!("{}", result);
}
