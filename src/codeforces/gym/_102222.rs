use std::io;
pub fn run() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let t: usize = buf.trim_end().parse().unwrap();
    for i in 0..t {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        let inputs =  buf.split_whitespace().collect::<Vec<&str>>();
        let n: usize = inputs[0].parse().unwrap();
        let k: usize = inputs[1].parse().unwrap();
        let q: i32 = inputs[2].parse().unwrap();
        let a = (1..n+1).map(|x| x as i32).collect::<Vec<i32>>();

    }
}

fn bubble_sort() {

}