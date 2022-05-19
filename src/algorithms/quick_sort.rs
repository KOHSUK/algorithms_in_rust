use std::io;

fn print_arr(arr: &[i32]) {
    let result = arr.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ");
    println!("{}", result);
}

pub fn run() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut arr = buf.split_whitespace().map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
    quick_sort(&mut arr);
    print_arr(&arr);
}

fn quick_sort(arr: &mut[i32]) {
    if arr.len() >= 2 {
        let p = partition(arr);
        quick_sort(&mut arr[..p]);
        quick_sort(&mut arr[p+1..]);
    }
}

fn partition(arr: &mut[i32]) -> usize {
    let mut i = 0;
    let pivot_idx = arr.len() - 1;
    let pivot = arr[pivot_idx];

    for j in 0..pivot_idx {
        if arr[j] < pivot {
            arr.swap(i, j);
            i+=1;
        }
    }
    arr.swap(i, pivot_idx);
    i
}

// fn quick_sort(arr: &mut Vec<i32>, low: usize, high: usize) {
//     if low < high {
//         let pi = partition(arr, low, high);
// 
//         quick_sort(arr, low, pi - 1);
//         quick_sort(arr, pi + 1, high)
//     }
// }
// 
// fn partition(arr: &mut Vec<i32>, low: usize, high: usize) -> usize {
//     let pivot = arr[high];
//     let mut i = low as i32 - 1;
// 
//     for j in low..high {
//         if arr[j] < pivot {
//             i+=1;
//             arr.swap(i as usize, j);
//         }
//     }
// 
//     arr.swap(( i+1 ) as usize, high);
// 
//     ( i+1 ) as usize
// }