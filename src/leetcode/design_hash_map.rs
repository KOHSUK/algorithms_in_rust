pub fn run() {
    let mut hash_map = MyHashMap::new();
    hash_map.put(1, 1);
    println!("null");
    hash_map.put(2, 2);
    println!("null");
    let val = hash_map.get(1);
    println!("{}", val);
    let val = hash_map.get(3);
    println!("{}", val);
    hash_map.put(2, 1);
    println!("null");
    let val = hash_map.get(2);
    println!("{}", val);
    hash_map.remove(2);
    println!("null");
    let val = hash_map.get(2);
    println!("{}", val);
}

const SIZE: i32 = 100003;

struct MyHashMap {
    buckets: [Option<i32>; SIZE as usize],
}

impl MyHashMap {
    fn new() -> Self {
        Self {
            buckets: [None; SIZE as usize],
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        let i = 0;
        let hash = Self::h(key, i);
        self.buckets[hash] = Some(value);
    }

    fn get(&self, key: i32) -> i32 {
        let i = 0;
        let hash = Self::h(key, i);
        self.buckets[hash].unwrap_or(-1)
    }

    fn remove(&mut self, key: i32) {
        let i = 0;
        let hash = Self::h(key, i);
        self.buckets[hash] = None;
    }

    fn h1(key: i32) -> i32 {
        key % SIZE
    }

    fn h2(key: i32) -> i32 {
        1 + key % (SIZE - 1)
    }

    fn h(key: i32, index: i32) -> usize {
        ((Self::h1(key) + index * Self::h2(key)) % SIZE) as usize
    }
}
