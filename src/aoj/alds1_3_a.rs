use std::io;

pub fn run() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let arr: Vec<&str> = buf.split_whitespace().collect::<Vec<&str>>();
    reverse_polish_notation(arr);
}

fn reverse_polish_notation(arr: Vec<&str>) {
    let mut stack = Vec::new();
    for item in arr {
        if let Ok(num) = item.parse::<i32>() {
            stack.push(num);
        } else {
            let num2 = stack.pop().unwrap(); 
            let num1 = stack.pop().unwrap(); 
            let result = match item {
                "+" => num1 + num2,
                "-" => num1 - num2,
                "*" => num1 * num2,
                _ => panic!("No way!")
            };
            stack.push(result);
        }
    }
    let res =  stack.pop().unwrap();
    println!("{}", res);
}