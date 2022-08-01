use std::fmt::Display;

struct MaxHeap {
    arr: Vec<i32>,
    size: usize,
    current_size: usize,
}

impl MaxHeap {
    fn new(size: usize) -> Self {
        Self {
            arr: vec![0],
            size,
            current_size: 0,
        }
    }

    fn heapify(&mut self, index: usize) {
        let left = index * 2;

        let right = index * 2 + 1;

        let mut max_index = index;
        if self.arr[left] > self.arr[index] {
            max_index = left;
        } else if right <= self.current_size && self.arr[right] > self.arr[max_index] {
            max_index = right;
        }

        if max_index != index {
            self.arr.swap(index, max_index);
            if max_index * 2 <= self.current_size {
                self.heapify(max_index);
            }
        }
    }

    fn add(&mut self, item: i32) {
        if self.current_size >= self.size {
            println!("Added too many elements");

            return;
        }
        self.arr.push(item);
        self.current_size += 1;
        for i in (1..=(self.current_size / 2)).rev() {
            self.heapify(i);
        }
    }

    fn peek(&self) -> Result<i32, String> {
        if self.current_size == 0 {
            return Err("No element".to_string());
        }
        Ok(self.arr[1])
    }

    fn pop(&mut self) -> Result<i32, String> {
        if self.current_size == 0 {
            return Err("No element".to_string());
        }
        self.arr.swap(1, self.current_size);
        self.current_size -= 1;
        for i in (1..=(self.current_size / 2)).rev() {
            self.heapify(i);
        }
        Ok(self.arr.pop().unwrap())
    }
}

impl Display for MaxHeap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "[{}]",
            self.arr
                .iter()
                .skip(1)
                .take(self.current_size)
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(",")
        ))
    }
}

pub fn run() {
    test_case_1();
}

fn test_case_1() {
    // Test case
    let mut maxheap = MaxHeap::new(3);
    maxheap.add(1);
    maxheap.add(2);
    maxheap.add(3);
    // [3,1,2]
    println!("{}", maxheap);
    // 3
    let res = maxheap.peek().map(|x| x.to_string()).unwrap_or_else(|x| x);
    println!("{}", res);
    // 3
    let res = maxheap.pop().map(|x| x.to_string()).unwrap_or_else(|x| x);
    println!("{}", res);
    // 2
    let res = maxheap.pop().map(|x| x.to_string()).unwrap_or_else(|x| x);
    println!("{}", res);
    // 1
    let res = maxheap.pop().map(|x| x.to_string()).unwrap_or_else(|x| x);
    println!("{}", res);
    // No element
    println!("{}", maxheap);
    maxheap.add(1);
    maxheap.add(2);
    maxheap.add(4);
    // Add too many elements
    maxheap.add(5);
    // [4,1,2]
    println!("{}", maxheap);
}
