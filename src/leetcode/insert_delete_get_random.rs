// rand = "0.8.5" を入れるとコンパイル終わらないのでコメントアウトしていますが、動作はOKのはず

// use rand::Rng;
// use std::collections::HashSet;
//
// struct RandomizedSet {
//     hash_set: HashSet<i32>,
// }
//
// /**
//  * `&self` means the method takes an immutable reference.
//  * If you need a mutable reference, change it to `&mut self` instead.
//  */
// impl RandomizedSet {
//     fn new() -> Self {
//         Self {
//             hash_set: HashSet::new(),
//         }
//     }
//
//     fn insert(&mut self, val: i32) -> bool {
//         self.hash_set.insert(val)
//     }
//
//     fn remove(&mut self, val: i32) -> bool {
//         self.hash_set.remove(&val)
//     }
//
//     fn get_random(&self) -> i32 {
//         let n = self.hash_set.len();
//         let mut rng = rand::thread_rng();
//         let rand_num: usize = rng.gen();
//         let index = rand_num % n;
//         let vec: Vec<_> = self.hash_set.clone().into_iter().collect();
//         vec[index]
//     }
// }
//
// pub fn run() {
//     let mut obj = RandomizedSet::new();
//     let ret: bool = obj.insert(3);
//     println!("insert(3): {}", ret);
//     let ret: bool = obj.insert(4);
//     println!("insert(4): {}", ret);
//     let ret: bool = obj.insert(5);
//     println!("insert(5): {}", ret);
//     let ret: bool = obj.insert(3);
//     println!("insert(3): {}", ret);
//     let ret: bool = obj.remove(4);
//     println!("remove(4): {}", ret);
//     let ret: i32 = obj.get_random();
//     println!("get_random(): {}", ret);
// }
//
