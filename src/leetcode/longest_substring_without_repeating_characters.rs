struct Solution;

use std::cmp::max;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut chars: [u8; 128] = [0; 128];
        let mut left = 0;
        let mut right = 0;
        let mut res = 0;

        while right < s.len() {
            if let Some(r) = s.chars().nth(right) {
                chars[r as usize] += 1;
                while chars[r as usize] > 1 {
                    if let Some(l) = s.chars().nth(left) {
                        chars[l as usize] -= 1;
                        left += 1;
                    }
                }
            }
            let len = right as i32 - left as i32 + 1;
            res = max(res, len);

            right += 1;
        }

        res
    }
}

pub fn run() {
    let s = "dvdf".to_string();
    let result = Solution::length_of_longest_substring(s);
    println!("{}", result);
}
