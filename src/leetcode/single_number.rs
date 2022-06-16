struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut hash_set = HashSet::new();
        for num in nums {
            if hash_set.contains(&num) {
                hash_set.remove(&num);
            } else {
                hash_set.insert(num);
            }
        }
        *hash_set.iter().collect::<Vec<&i32>>()[0]
    }
}

pub fn run() {
    let nums = vec![4, 1, 2, 1, 2];
    let result = Solution::single_number(nums);
    println!("{}", result);
}
