use std::io;

fn print_arr(arr: &[i32]) {
    let result = arr.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ");
    println!("{}", result);
}

pub fn run() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut nums1 = buf.split_whitespace().map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let m: i32 = buf.trim_end().parse().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut nums2 = buf.split_whitespace().map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let n: i32 = buf.trim_end().parse().unwrap();
    Solution::merge(&mut nums1, m, &mut nums2, n);
    print_arr(&nums1);
}

struct Solution {}

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n:i32) {
        let mut new_nums = Vec::new();
        let m = m as usize;
        let n = n as usize;
        let mut i = 0usize;
        let mut j = 0usize;
        for k in 0..nums1.len() {
            let mut l_item = i32::MAX;
            let mut r_item = i32::MAX;

            if i < m {
                l_item = nums1[i];
            }

            if j < n {
                r_item = nums2[j];
            }

            if l_item < r_item {
                new_nums.push(l_item);
                i+=1;
            } else {
                new_nums.push(r_item);
                j+=1;
            }
        }

        for i in 0..nums1.len() {
            nums1[i] = new_nums[i];
        }
    }
}