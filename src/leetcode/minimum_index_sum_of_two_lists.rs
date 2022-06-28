struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let mut hash_map = HashMap::new();
        for (index, restaurant) in list1.iter().enumerate() {
            hash_map.insert(restaurant, index);
        }

        let mut min: (usize, String) = (9999, "None".to_string());
        let mut result = Vec::new();
        for (index, restaurant) in list2.iter().enumerate() {
            if let Some(&idx1) = hash_map.get(restaurant) {
                let sum = index + idx1;
                if sum < min.0 {
                    min = (sum, restaurant.to_string());
                    result = vec![min.clone()]
                } else if sum == min.0 {
                    min = (sum, restaurant.to_string());
                    result.push(min.clone());
                }
            }
        }

        result.iter().map(|v| v.1.to_owned()).collect()
    }
}

pub fn run() {
    let list1 = vec!["Shogun", "Tapioca Express", "Burger King", "KFC"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let list2 = vec![
        "Piatti",
        "The Grill at Torrey Pines",
        "Hungry Hunter Steakhouse",
        "Shogun",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    let result = Solution::find_restaurant(list1, list2);
    println!("{:?}", result);
}
