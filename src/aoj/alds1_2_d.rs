use std::io;

pub fn run() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim_end().parse().unwrap();
    let mut arr: Vec<i32> = Vec::new();
    for _ in 0..n {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        let num: i32 = buf.trim_end().parse().unwrap();
        arr.push(num);
    }
    shell_sort(&mut arr, n);
}

fn print_arr(arr: &[i32]) {
    for n in arr.iter() {
        println!("{}", n);
    }
}

fn print_usize_arr(arr: &[usize]) {
    let value = arr.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ");
    println!("{}", value);
}

fn shell_sort(arr: &mut Vec<i32>, n: usize) {
    let mut gaps = Vec::new();

    for i in (0..10).rev() {
        let value = 4_i32.pow(i) as usize;
        if value <= n {
            gaps.push(value);
        }
    }

    let mut cnt = 0;
    for g in gaps.iter() {
        cnt += insert(arr, n, *g);
    }
    println!("{}", gaps.len());
    print_usize_arr(&gaps);
    println!("{}", cnt);
    print_arr(arr);
}

fn insert(arr: &mut Vec<i32>, n: usize, g: usize) -> i32 {
    let mut cnt = 0;
    for i in g..n {
        let mut j = i - g;
        let target = arr[i];
        while target < arr[j] {
            arr[j + g] = arr[j];
            arr[j] = target;
            cnt+=1;

            if j < g {
                break;
            }

            j-=g;            
        }
    }
    cnt
}