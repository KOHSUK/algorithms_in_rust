use std::io;

// Input:
// 1 2 4
// 1 3 4
// Output:
// 1 1 2 3 4 4

fn print_list(list: &Option<Box<ListNode>>) {
    let mut list = list;
    loop {
        let node = list;
        if node.is_none() {
            break;
        }
        print!("{}", node.as_ref().unwrap().val);
        list = &node.as_ref().unwrap().next;
    }
    println!();
}

pub fn run() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let values = buf.split_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<i32>>();
    let list1 = Some(Box::new(ListNode::from_vec(values)));

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let values = buf.split_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<i32>>();
    let list2 = Some(Box::new(ListNode::from_vec(values)));

    let result = Solution::merge_two_lists(list1, list2);
    print_list(&result);
}

struct Solution {}

#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

impl ListNode {
    fn from_vec(v: Vec<i32>) -> Self {
        let mut list_nodes = v.iter().map(|x|
            ListNode::new(*x)
        ).collect::<Vec<ListNode>>();

        for i in (1..list_nodes.len()).rev() {
            list_nodes[i-1].next = Some(Box::new(list_nodes[i].clone()));
        }

        list_nodes[0].clone()
    }
}

impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if list1.is_none() && list2.is_none() {
            return None
        }
        let mut new_arr = Vec::new();
        let mut list1 = list1;
        let mut list2 = list2;
        let mut end = false;
        while !end {
            let l_item = match list1.as_ref() {
                Some(node) => node.val,
                _ => i32::MAX,
            };

            let r_item = match list2.as_ref() {
                Some(node) => node.val,
                _ => i32::MAX,
            };

            if l_item < r_item {
                new_arr.push(l_item);
                list1 = match list1.as_ref() {
                    Some(node) => node.next.clone(),
                    None => None
                }
            } else {
                new_arr.push(r_item);
                list2 = match list2.as_ref() {
                    Some(node) => node.next.clone(),
                    None => None
                }
            }

            if list1.is_none() && list2.is_none() {
                end = true;
            }
        }

        Some(Box::new(ListNode::from_vec(new_arr)))
    }
}