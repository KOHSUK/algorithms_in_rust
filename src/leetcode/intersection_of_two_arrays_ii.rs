struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut hash_map = HashMap::new();

        for num in nums1 {
            let mut count = 1;
            if let Some(cnt) = hash_map.get(&num) {
                count = cnt + 1
            }
            hash_map.insert(num, count);
        }

        let mut result = Vec::new();
        for num in nums2 {
            if let Some(&cnt) = hash_map.get(&num) {
                if cnt > 0 {
                    result.push(num);
                }
                hash_map.insert(num, cnt - 1);
            }
        }

        result
    }
}

pub fn run() {
    let nums1 = vec![1, 2, 2, 1];
    let nums2 = vec![2, 2];
    let result = Solution::intersect(nums1, nums2);
    println!("{:?}", result);
}
