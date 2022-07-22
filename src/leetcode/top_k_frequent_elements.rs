struct Solution;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Eq, PartialEq, Clone, Copy)]
struct Item {
    num: i32,
    count: i32,
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        other.count.partial_cmp(&self.count).unwrap()
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.count.partial_cmp(&other.count) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        other.count.partial_cmp(&self.count)
    }
}

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

        let mut heap = map_count
            .into_iter()
            .map(|(num, count)| Item { num, count })
            .fold(BinaryHeap::new(), |mut acc, item| {
                acc.push(item);
                acc
            });

        let mut result = Vec::new();
        for _ in 0..k {
            result.push(heap.pop().unwrap().num);
        }

        result
    }
}

pub fn run() {
    let nums = vec![9, 9, 9, 5, 2, 2, 2, 2, 2, 2, 4, 4, 4, 4, 0, 0];
    let k = 2;
    let result = Solution::top_k_frequent(nums, k);
    println!("{:?}", result);
}
