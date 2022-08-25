use std::io;
fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();
    for _ in 0..n {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        let mut list = buf.split_whitespace().map(|x| x.parse::<usize>().unwrap());
        let u = list.next().unwrap();
        let _k = list.next().unwrap();

        let mut row = std::iter::repeat(0).take(n).collect::<Vec<i32>>();
        for node in list {
            row[node - 1] = 1;
        }
        let result = row
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        println!("{}", result);
    }
}

pub fn run() {
    main();
}
