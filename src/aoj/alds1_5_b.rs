use std::io;

pub fn run() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim_end().parse().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut arr = buf.split_whitespace().map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
    let count = merge_sort(&mut arr, 0, n-1);
    print_arr(&arr);
    println!("{}", count);
}

fn merge_sort(arr: &mut Vec<i32>, l: usize, r: usize) -> i32 {
    if l >= r {
        return 0;
    }

    let mut count = 0;

    let m = l + (r-l) / 2;

    count += merge_sort(arr, l, m);
    count += merge_sort(arr, m+1, r);

    count += merge(arr, l, m, r);

    count
}

fn merge(arr: &mut Vec<i32>, l: usize, m: usize, r: usize) -> i32 {
    let mut i = l;
    let mut j = m+1;
    let mut new_arr = Vec::new();

    let mut count = 0;
    for _ in l..r+1 {
        let mut l_item = i32::MAX;// (i32::max_value();) for older version
        let mut r_item = i32::MAX;

        if i <= m {
            l_item = arr[i];
        }

        if j <= r {
            r_item = arr[j];
        }

        if l_item < r_item {
            new_arr.push(l_item);
            i+=1;
        } else {
            new_arr.push(r_item);
            j+=1;
        }

        count += 1;
    }

    let mut i = 0;
    for j in l..r+1 {
        arr[j] = new_arr[i];
        i+=1;
    }

    count
}

fn print_arr(arr: &[i32]) {
    let result = arr.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ");
    println!("{}", result);
}