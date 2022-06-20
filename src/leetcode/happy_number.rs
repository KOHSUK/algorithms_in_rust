struct Solution;

fn print_arr(arr: &[i32], res: i32) {
    let str = arr
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("^2 + ");
    let str = format!("{}^2 = {}", str, res);
    println!("{}", str);
}

use std::collections::HashSet;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut hash_set = HashSet::new();
        hash_set.insert(n);
        let mut num = n;
        loop {
            let n = num.to_string();
            let digits = n
                .chars()
                .into_iter()
                .map(|n| n.to_string().parse().unwrap())
                .collect::<Vec<i32>>();
            let sum = digits.iter().map(|n| n.pow(2)).sum();
            print_arr(&digits, sum);
            if sum == 1 {
                break true;
            }
            if hash_set.contains(&sum) {
                break false;
            }
            hash_set.insert(sum);
            num = sum;
        }
    }
}

pub fn run() {
    let res = Solution::is_happy(19);
    println!("{}", res);
}
