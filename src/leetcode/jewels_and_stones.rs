struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        // let jewels: HashSet<char> = HashSet::from_iter(jewels.chars());
        // Under the circumstances where you cannot use from_iter, use the code below
        let jewels = jewels.chars().fold(HashSet::new(), |mut acc, value| {
            acc.insert(value);
            acc
        });

        let mut cnt = 0;
        for stone in stones.chars() {
            if jewels.contains(&stone) {
                cnt += 1;
            }
        }
        cnt
    }
}

pub fn run() {
    let jewels = "aA".to_string();
    let stones = "aAAbbbb".to_string();
    let result = Solution::num_jewels_in_stones(jewels, stones);
    println!("{}", result);
}
