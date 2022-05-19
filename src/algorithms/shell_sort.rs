use std::io;

pub fn run() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut arr: Vec<i32> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let gaps = generate_gaps(arr.len());
    shell_sort(&mut arr, &gaps);
    print_arr(&arr);
}

fn print_arr<T: std::fmt::Display>(arr: &[T]) {
    let result = arr.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ");
    println!("{}", result);
}

fn generate_gaps(n: usize) -> Vec<usize> {
    let mut gaps = vec![0];
    for k in 1..10 {
        let gap = 4_usize.pow(k) + 3 * 2_usize.pow(k) + 1;
        if gap < n {
            gaps.push(gap);
        } else {
            break;
        }
    }

    gaps.reverse();
    gaps
}

fn shell_sort(arr: &mut Vec<i32>, gaps: &[usize]) {
    for &g in gaps {
        insertion_sort(arr, g);
    }
}

fn insertion_sort(arr: &mut Vec<i32>, gap: usize) {
    for i in gap..arr.len() {
        let temp = arr[i];
        let mut j = i;
        while j >= gap && arr[j - gap] > temp {
            arr[j] = arr[j - gap];
            j -= gap;
        }
        arr[j] = temp;
    }
}