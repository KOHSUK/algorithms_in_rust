struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;
        let mut hash_map = HashMap::new();
        for (index, num) in nums.iter().enumerate() {
            if let Some(idx) = hash_map.get(num) {
                if index - idx <= k {
                    return true;
                }
            }
            hash_map.insert(*num, index);
        }

        false
    }
}

pub fn run() {
    let nums = vec![1, 2, 3, 1, 2, 3];
    let k = 2;
    let result = Solution::contains_nearby_duplicate(nums, k);
    println!("{}", result);
}
