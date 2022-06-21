// https://leetcode.com/explore/learn/card/hash-table/184/comparison-with-other-data-structures/1117/

struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut s_dict = HashMap::new();
        let mut t_dict = HashMap::new();

        let mut result = true;
        for index in 0..s.len() {
            // この段階でNoneにはならないのでunwrapを許す
            let s_char = s.chars().nth(index).unwrap();
            let t_char = t.chars().nth(index).unwrap();

            let s_res = s_dict.get(&s_char);
            let t_res = t_dict.get(&t_char);

            if (s_res.is_some() && t_res.is_none()) || (s_res.is_none() && t_res.is_some()) {
                result = false;
                break;
            }

            if let Some(&s_idx) = s_res {
                if let Some(&t_idx) = t_res {
                    if s_idx != t_idx {
                        result = false;
                        break;
                    }
                }
            }
            s_dict.insert(s_char, index);
            t_dict.insert(t_char, index);
        }

        result
    }
}

pub fn run() {
    let s = "foo".to_string();
    let t = "bar".to_string();
    let result = Solution::is_isomorphic(s, t);
    println!("{}", result);
}
