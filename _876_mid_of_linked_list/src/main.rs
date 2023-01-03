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

struct Solution {}

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let head = &head;
        //V1 two iterations
        // if head.is_none() {
        //     return head;
        // }

        // let mut cnt = 0;
        // let mut curr = head;
        // while let Some(node) = curr {
        //     cnt += 1;
        //     curr = &node.next;
        // }

        // cnt = cnt / 2;

        // let mut curr = head;
        // for i in 0..cnt {
        //     if let Some(node) = curr {
        //         curr = &node.next;
        //     }
        // }

        // curr;

        //V2 slow and fast pointers
        let mut slow = head;
        let mut fast = head;
        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            slow = &(slow.as_ref().unwrap().next);
            fast = &(fast.as_ref().unwrap().next.as_ref().unwrap().next);
        }
        slow.to_owned()
    }
}

fn main() {
    let input = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode {
                        val: 5,
                        next: Some(Box::new(ListNode { val: 6, next: None })),
                    })),
                })),
            })),
        })),
    }));
    let result = Solution::middle_node(input);
    println!("{result:?}");

    assert_eq!(
        Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode {
                val: 5,
                next: Some(Box::new(ListNode { val: 6, next: None })),
            })),
        })),
        result
    );

    let input = Some(Box::new(ListNode {
        val: 33,
        next: None,
    }));
    let result = Solution::middle_node(input);
    println!("{result:?}");

    assert_eq!(
        Some(Box::new(ListNode {
            val: 33,
            next: None,
        })),
        result
    );
}
