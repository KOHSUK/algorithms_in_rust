use std::io;
struct Solution {}

// https://leetcode.com/problems/sort-an-array/submissions/
// input: "5 2 3 1"

pub fn run() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let arr: Vec<i32> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect::<Vec<i32>>();

    let nums = Solution::sort_array(arr);
    dbg!(nums);
}

impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        shell_sort(nums)
    }
}

fn selection_sort(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    let mut sorted = nums.len();
    for _ in 0..(nums.len()-1) {
        let mut max = 0;
        for i in 0..sorted {
            if nums[i] > nums[max] {
                max = i;
            }
        }
        sorted -= 1;
        nums.swap(max, sorted);
    }
    nums
}

fn shell_sort(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    let mut gaps = Vec::new();
    for i in ( 0..10 ).rev() {
        let gap = 4 * i + 3 * 2 * i + 1;
        if gap < nums.len() {
            gaps.push(gap);
        }
    };

    for g in gaps {
        for i in g..nums.len() {
            let mut j = i;
            let target = nums[i];
            while j >= g && target < nums[j-g] {
                nums[j] = nums[j - g];
                j -= g;
            }
            nums[j] = target;
        }
    }

    nums

}