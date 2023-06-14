//Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn append(&mut self, val: i32) {
        match self.next {
            Some(ref mut next) => {
                ListNode::append(next, val);
            }
            None => {
                self.next = Some(Box::new(ListNode::new(val)));
            }
        }
    }
}

impl From<&[i32]> for ListNode {
    fn from(arr: &[i32]) -> Self {
        let mut head = ListNode::new(arr[0]);
        arr.iter().skip(1).for_each(|val| head.append(*val));
        head
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

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result = Box::new(ListNode::new(0));
        let mut head = result.as_mut();
        let mut l1 = &l1;
        let mut l2 = &l2;
        let mut carry = 0;
        while l1.is_some() || l2.is_some() || carry != 0 {
            let mut l = 0;
            if let Some(v1) = l1 {
                l = v1.val;
                l1 = &l1.as_ref().unwrap().next;
            }
            let mut r = 0;
            if let Some(v2) = l2 {
                r = v2.val;
                l2 = &l2.as_ref().unwrap().next;
            }

            let sum = carry + l + r;
            carry = sum / 10;
            head.next = Some(Box::new(ListNode::new(sum % 10)));
            head = head.next.as_mut().unwrap();
        }
        result.next
    }
}

fn main() {
    let l1 = Some(Box::new(ListNode::from(&[2, 4, 3][..])));
    let l2 = Some(Box::new(ListNode::from(&[5, 6, 4][..])));

    let res = Solution::add_two_numbers(l1, l2);
    print_list_node(&res);
    println!("__");

    let l1 = Some(Box::new(ListNode::from(&[9, 9, 9, 9, 9, 9, 9][..])));
    let l2 = Some(Box::new(ListNode::from(&[9, 9, 9, 9][..])));

    let res = Solution::add_two_numbers(l1, l2);
    print_list_node(&res);
    println!("__");

    let l1 = Some(Box::new(ListNode::from(&[9][..])));
    let l2 = Some(Box::new(ListNode::from(&[9][..])));

    let res = Solution::add_two_numbers(l1, l2);
    print_list_node(&res);
    println!("__");
}
