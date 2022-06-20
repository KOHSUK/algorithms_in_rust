struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut hash_set = HashSet::new();
        for num in nums1 {
            hash_set.insert(num);
        }
        let mut result = HashSet::new();
        for num in nums2 {
            if hash_set.contains(&num) {
                result.insert(num);
            }
        }
        result.into_iter().collect::<Vec<i32>>()
    }
}

pub fn run() {
    let nums1 = vec![4, 9, 5];
    let nums2 = vec![9, 4, 9, 8, 4];
    let res = Solution::intersection(nums1, nums2);
    println!("{:?}", res);
}
