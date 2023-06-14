use std::mem;

//Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution;

impl Solution {
    fn merge_internal(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut tail = head.as_mut().unwrap();
        let mut l1 = list1.as_ref();
        let mut l2 = list2.as_ref();
        while l1.is_some() && l2.is_some() {
            let val1 = l1.unwrap().val;
            let val2 = l2.unwrap().val;
            if val1 <= val2 {
                tail.next = Some(l1.unwrap().clone());
                l1 = l1.unwrap().next.as_ref();
            } else {
                tail.next = Some(l2.unwrap().clone());
                l2 = l2.unwrap().next.as_ref();
            }
            tail = tail.next.as_mut().unwrap();
        }
        if l1.is_none() {
            l1 = l2;
        }
        if l1.is_some() {
            let end = Some(l1.unwrap().clone());
            tail.next = end;
        }

        return head.unwrap().next;
    }

    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Solution::merge_internal(list1, list2)
    }
}

fn print_list_node(list: &Option<Box<ListNode>>) {
    match list {
        Some(bx) => {
            print!("{:2} ", bx.val);
            print_list_node(&bx.next);
        }
        None => (),
    }
}

fn main() {
    //[1,2,4,5]
    //[1,3,4]
    let list_one = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode { val: 5, next: None })),
            })),
        })),
    }));

    let list_two = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));

    print_list_node(&list_one);
    println!("\n\r___");
    print_list_node(&list_two);
    println!("\n\r___");

    let result = Solution::merge_two_lists(list_one, list_two);
    print_list_node(&result);
}
