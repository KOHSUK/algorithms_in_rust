use std::{io, fmt::Display};

pub fn run() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut arr: Vec<usize> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let answer = counting_sort(arr);
    print_arr(&answer);
}

fn print_arr(arr: &[usize]) {
    for ( i, &item ) in arr.iter().enumerate() {
        print!("{}", item);
        if i < arr.len() - 1 {
            print!(" ");
        }
    }
    println!();
}

fn counting_sort(arr: Vec<usize>) -> Vec<usize> {
    let mut counts = vec![0; 10_000];
    for item in arr.iter() {
        counts[(*item) as usize]+=1;
    }

    for i in 1..counts.len() {
        counts[i] += counts[i-1];
    }

    let mut answer: Vec<usize> = vec![0; arr.len()];
    for &item in arr.iter().rev() {
        answer[counts[item]-1] = item;
        counts[item] -= 1;
    }

    answer
}