use std::io;

pub fn run() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim_end().parse().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut arr: Vec<i32> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect();
    selection_sort(&mut arr, n);
}

fn selection_sort(arr: &mut Vec<i32>, n: usize) {
    let mut count = 0;
    for i in 0..n-1 {
        count += select(arr, i);
    }
    print_arr(arr);
    println!("{}", count);
}

fn select(arr: &mut Vec<i32>, i: usize) -> i32 {
    let mut smallest = i;
    for j in i+1..arr.len() {
        if arr[j] < arr[smallest] {
            smallest = j;
        }
    }
    arr.swap(smallest, i);
    if smallest != i {
        1
    } else {
        0
    }
}

fn print_arr(arr: &[i32]) {
    let result = arr.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ");
    println!("{}", result);
}