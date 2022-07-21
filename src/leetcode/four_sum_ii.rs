struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn four_sum_count(
        nums1: Vec<i32>,
        nums2: Vec<i32>,
        nums3: Vec<i32>,
        nums4: Vec<i32>,
    ) -> i32 {
        let mut hash_map = HashMap::new();
        let n = nums1.len();
        for index1 in 0..n {
            let num1 = nums1[index1];
            for index2 in 0..n {
                let num2 = nums2[index2];
                let key = num1 + num2;
                let mut count = 1;
                if let Some(cnt) = hash_map.get(&key) {
                    count += cnt;
                }
                hash_map.insert(key, count);
            }
        }

        let mut result = 0;
        for index3 in 0..n {
            let num3 = nums3[index3];
            for index4 in 0..n {
                let num4 = nums4[index4];
                let key = -(num3 + num4);
                if let Some(cnt) = hash_map.get(&key) {
                    result += cnt;
                }
            }
        }

        result
    }
}

pub fn run() {
    let nums1 = vec![1, 2];
    let nums2 = vec![-2, -1];
    let nums3 = vec![-1, 2];
    let nums4 = vec![0, 2];
    let res = Solution::four_sum_count(nums1, nums2, nums3, nums4);
    println!("{}", res);
}
