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
    fn append(current: &mut ListNode, val: i32) {
        match current.next {
            Some(ref mut next) => {
                Solution::append(next, val);
            }
            None => {
                current.next = Some(Box::new(ListNode::new(val)));
            }
        }
    }

    fn merge_internal(
        list1: &Option<Box<ListNode>>,
        list2: &Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head;

        let mut l1_ptr = list1;
        let mut l2_ptr = list2;

        let val1 = l1_ptr.as_ref()?.val;
        let val2 = l2_ptr.as_ref()?.val;

        if val1 <= val2 {
            head = ListNode::new(val1);
            l1_ptr = &list1.as_ref()?.next;
        } else {
            head = ListNode::new(val2);
            l2_ptr = &list2.as_ref()?.next;
        }

        let mut tail = &mut head;

        loop {
            match (l1_ptr, l2_ptr) {
                (None, None) => break,
                (None, Some(l2)) => {
                    Solution::append(&mut tail, l2.val);
                    tail = &mut *tail.next.as_mut().unwrap();
                    l2_ptr = &l2_ptr.as_ref()?.next;
                }
                (Some(l1), None) => {
                    Solution::append(&mut tail, l1.val);
                    tail = &mut *tail.next.as_mut().unwrap();
                    l1_ptr = &l1_ptr.as_ref()?.next;
                }
                (Some(l1), Some(l2)) => {
                    if l1.val <= l2.val {
                        Solution::append(&mut tail, l1.val);
                        tail = &mut *tail.next.as_mut().unwrap();
                        l1_ptr = &l1_ptr.as_ref()?.next;
                    } else {
                        Solution::append(&mut tail, l2.val);
                        tail = &mut *tail.next.as_mut().unwrap();
                        l2_ptr = &l2_ptr.as_ref()?.next;
                    }
                }
            }
        }

        return Some(Box::new(head));
    }

    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (&list1, &list2) {
            (Some(_), Some(_)) => Solution::merge_internal(&list1, &list2),
            (None, None) => return None,
            (None, Some(_)) => return list2,
            (Some(_), None) => return list1,
        }
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

fn arr_to_list(arr: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    for n in arr {
        let mut new_node = ListNode::new(*n);
        new_node.next = head;
        head = Some(Box::new(new_node));
    }
    head
}

fn main() {
    //[1,2,4]
    //[1,3,4]
    let list_one = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));

    let list_two = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));

    let node = arr_to_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    print_list_node(&node);
    println!("\n\r___");

    print_list_node(&list_one);
    println!("\n\r___");
    print_list_node(&list_two);
    println!("\n\r___");

    let result = Solution::merge_two_lists(list_one, list_two);
    print_list_node(&result);
}
