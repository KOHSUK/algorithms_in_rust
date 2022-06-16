mod algorithms;
#[allow(unused)]
mod aoj;
mod codeforces;
mod leetcode;

use algorithms::*;
use aoj::*;
use codeforces::gym::*;
use leetcode::*;

fn main() {
    leetcode::contains_duplicate::run();
}
