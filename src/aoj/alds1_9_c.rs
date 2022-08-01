use std::collections::BinaryHeap;
use std::io;

pub fn run() {
    let mut pq = PriorityQueue::new();

    loop {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        let buf = buf.trim();
        if buf.eq_ignore_ascii_case("end") {
            break;
        } else if buf.eq_ignore_ascii_case("extract") {
            let res = pq.extractMax();
            println!("{}", res);
        } else {
            let val: i32 = buf.split_whitespace().collect::<Vec<&str>>()[1]
                .parse()
                .unwrap();
            pq.insert(val);
        }
    }
}

struct PriorityQueue {
    heap: BinaryHeap<i32>,
}

impl PriorityQueue {
    fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
        }
    }

    fn insert(&mut self, x: i32) {
        self.heap.push(x);
    }

    fn extractMax(&mut self) -> i32 {
        self.heap.pop().unwrap()
    }
}
