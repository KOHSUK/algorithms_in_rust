// https://leetcode.com/explore/learn/card/hash-table/184/comparison-with-other-data-structures/1115/

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hash_map = HashMap::new();
        let mut result = Vec::new();
        for (index, num) in nums.iter().enumerate() {
            if let Some(idx) = hash_map.get(num) {
                result.push(*idx as i32);
                result.push(index as i32);
            }
            let sub = target - *num;
            hash_map.insert(sub, index);
        }
        result
    }
}

pub fn main() {
    let nums = vec![3, 2, 4];
    let target = 6;
    let result = Solution::two_sum(nums, target);
    println!("{:?}", result);
}
