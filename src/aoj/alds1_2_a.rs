use std::io;

fn print_arr(arr: &[i32]) {
    let arr: String = arr.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ");
    println!("{}", arr);
}

pub fn run() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim_end().parse().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut arr: Vec<i32> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let swaps = bubble_sort(n, &mut arr);
    print_arr(&arr);
    println!("{}", swaps);
}

fn bubble_sort(n: usize, arr: &mut Vec<i32>) -> i32 {
    let mut swaps = 0;
    loop {
        let mut count = 0;
        for i in (1..n).rev() {
            if arr[i] < arr[i-1] {
                arr.swap(i, i-1);
                swaps += 1;
                count += 1;
            }
        }
        if count == 0 {
            break;
        }
    }
    swaps
}