struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map_count = HashMap::new();

        for num in nums {
            let mut count = 1;
            if let Some(cnt) = map_count.get(&num) {
                count += cnt;
            }
            map_count.insert(num, count);
        }

        let mut counts = map_count.into_iter().collect::<Vec<(i32, i32)>>();
        counts.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        counts.iter().take(k as usize).map(|x| x.0).collect()
    }
}

pub fn run() {
    let nums = vec![9, 9, 9, 5, 2, 2, 2, 2, 2, 2, 4, 4, 4, 4, 0, 0];
    let k = 2;
    let result = Solution::top_k_frequent(nums, k);
    println!("{:?}", result);
}
