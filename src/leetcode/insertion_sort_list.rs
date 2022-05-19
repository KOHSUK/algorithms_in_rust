use std::io;

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

// input: "4 2 1 3"


// TODO: この問題は未完了

// answer from https://leetcode.com/problems/insertion-sort-list/discuss/452139/Rust

pub struct Solution {}

impl Solution {
    pub fn run() {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        let values = buf.split_whitespace().map(|x| x.parse().unwrap()).collect::<Vec<i32>>();

        let mut head = ListNode::new(0);
        let mut ptr = &mut head;
        for value in values.iter() {
            let next = ListNode::new(*value);
            ptr.next = Some(Box::new(next));
            // ptr = &mut ptr.next.as_ref().unwrap();
        }

        Self::insertion_sort_list(head.next);
    }

    fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut new_head = ListNode::new(0);
        while let Some(mut boxed) = head.take() {
            head = boxed.next.take();
            let mut ptr = &mut new_head;
            // ptr.next should be the first element bigger than or equal to boxed.val or None.
            while ptr.next.as_ref().is_some() && ptr.next.as_ref().unwrap().val < boxed.val {
                ptr = ptr.next.as_mut().unwrap();
            }
            boxed.next = ptr.next.take();
            ptr.next = Some(boxed);
        }
        new_head.next
    }
}