struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut hash_map = HashMap::new();
        for (index, char) in s.chars().enumerate() {
            let mut count = 1;
            if let Some(cnt) = hash_map.get(&char) {
                count = cnt + 1;
            }
            hash_map.insert(char, count);
        }

        let mut res = -1;
        for (index, char) in s.chars().enumerate() {
            if let Some(&count) = hash_map.get(&char) {
                if count == 1 {
                    res = index as i32;
                    break;
                }
            }
        }
        res
    }
}

pub fn run() {
    let result = Solution::first_uniq_char("loveleetcode".to_string());
    println!("{}", result);
}
