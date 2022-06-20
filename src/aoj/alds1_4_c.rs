use std::collections::HashMap;
use std::io;

pub fn run() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let num: usize = buf.trim().parse().unwrap();

    let mut dict: HashMap<String, String> = HashMap::new();
    let mut inputs = Vec::new();
    for _ in 0..num {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        inputs.push(buf.to_string());
    }

    for input in inputs {
        let input = input
            .split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        exec(&input[0], &input[1], &mut dict);
    }
}

fn exec(operator: &str, operand: &str, dict: &mut HashMap<String, String>) {
    match operator {
        "insert" => {
            dict.insert(operand.to_string(), operand.to_string());
        }
        "find" => {
            let res = dict.get(operand).and(Some("yes")).or(Some("no")).unwrap();
            println!("{}", res);
        }
        _ => panic!("Error"),
    }
}
