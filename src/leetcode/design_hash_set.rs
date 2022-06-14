// https://leetcode.com/explore/learn/card/hash-table/182/practical-applications/1139/

pub fn run() {
    let mut hash_set = MyHashSet::new();

    hash_set.add(1);
    println!("hash_set.add(1)");
    hash_set.add(1);
    println!("hash_set.add(1)");
    hash_set.add(2);
    println!("hash_set.add(2)");
    hash_set.add(3);
    println!("hash_set.add(3)");
    hash_set.remove(2);
    println!("hash_set.remove(2)");
    println!("hash_set.contains(3): {}", hash_set.contains(3));
    println!("hash_set.contains(2): {}", hash_set.contains(2));
}

// `key` ranges from 0 to 1000000
// the bucket size is up to 1000

// a prime number greater than 1000
const SIZE: usize = 1009;

struct MyHashSet {
    buckets: [Option<i32>; SIZE],
}

// Open addressing method using double hashes.

impl MyHashSet {
    fn new() -> Self {
        Self {
            buckets: [None; SIZE],
        }
    }

    fn add(&mut self, key: i32) {
        for i in 0..SIZE {
            let hash = Self::h(key, i as i32);
            if let Some(value) = self.buckets[hash] {
                if value == key {
                    break;
                }
            } else {
                self.buckets[hash] = Some(key);
                break;
            }
        }
    }

    fn remove(&mut self, key: i32) {
        for i in 0..SIZE {
            let hash = Self::h(key, i as i32);
            if let Some(value) = self.buckets[hash] {
                if value == key {
                    self.buckets[hash] = None;
                    break;
                }
            }
        }
    }

    fn contains(&self, key: i32) -> bool {
        let mut res = false;
        for i in 0..SIZE {
            let hash = Self::h(key, i as i32);
            if let Some(value) = self.buckets[hash] {
                if value == key {
                    res = true;
                }
            }
        }
        res
    }

    fn h1(key: i32) -> usize {
        (key % (SIZE as i32)) as usize
    }

    fn h2(key: i32) -> usize {
        (1 + key % ((SIZE as i32) - 1)) as usize
    }

    fn h(key: i32, i: i32) -> usize {
        let i = i as usize;
        (Self::h1(key) + i * Self::h2(key)) % SIZE
    }
}
