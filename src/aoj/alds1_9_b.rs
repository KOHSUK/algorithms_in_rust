use std::io;

fn print_arr(arr: &[i32]) {
    let res: Vec<_> = arr[1..].iter().map(|x| x.to_string()).collect();
    println!(" {}", res.join(" "));
}

pub fn run() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let h: usize = buf.trim().parse().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let arr: Vec<i32> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut arr = [vec![0], arr].concat().to_vec();
    build_heap(&mut arr, h);
    print_arr(&arr);
}

fn left(i: usize) -> usize {
    i * 2
}

fn right(i: usize) -> usize {
    i * 2 + 1
}

fn build_heap(arr: &mut Vec<i32>, h: usize) {
    for i in (1..=(h / 2)).rev() {
        max_heapify(arr, i, h);
    }
}

fn max_heapify(arr: &mut Vec<i32>, i: usize, h: usize) {
    let l = left(i);
    let r = right(i);
    let mut largest = 0;
    if l <= h && arr[l] > arr[i] {
        largest = l;
    } else {
        largest = i;
    }
    if r <= h && arr[r] > arr[largest] {
        largest = r;
    }

    if largest != i {
        arr.swap(i, largest);
        max_heapify(arr, largest, h)
    }
}
