struct Solution;

use std::collections::HashMap;

fn get_sorted_key(key: &str) -> String {
    let mut chars: Vec<_> = key.chars().collect();
    chars.sort();
    let key: String = chars.into_iter().collect();
    key
}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut hash_map: HashMap<String, Vec<String>> = HashMap::new();
        for str in strs {
            let key = get_sorted_key(&str);
            let group = hash_map.get(&key);
            if group.is_some() {
                let mut group = group.unwrap().clone();
                group.push(str);
                hash_map.insert(key, group);
            } else {
                hash_map.insert(key, vec![str]);
            }
        }

        hash_map.values().cloned().collect::<Vec<Vec<String>>>()
    }
}

pub fn run() {
    let strs = vec!["eat", "tea", "tan", "ate", "nat", "bat"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let result = Solution::group_anagrams(strs);
    println!("{:?}", result);
}
