use std::io;

pub fn run() {
    insertion_sort();
}

fn print_arr(arr: &[i32]) {
    let result = arr.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ");
    println!("{}", result);
}

fn insertion_sort() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let n : usize = buf.trim_end().parse().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut arr: Vec<i32> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let mut i: usize = 1;
    while i < n {
        print_arr(&arr);
        insert(&mut arr, i);
        i+=1;
    }
    print_arr(&arr);
}

fn insert(arr: &mut Vec<i32>, i: usize) {
    let target = arr[i];
    let mut j = i - 1;
    loop {
        if target < arr[j] {
            arr[j+1] = arr[j];
            arr[j] = target;
        } else {
            break;
        }
        if j == 0 {
            break
        }
        j-=1;
    }
}