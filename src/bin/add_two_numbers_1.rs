// Definition for singly-linked list.
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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut carry = 0;
        let mut l1 = l1;
        let mut l2 = l2;
        let mut head = Box::new(ListNode::new(0));
        let mut tail = &mut head;
        while l1.is_some() || l2.is_some() {
            let l1_current_node = l1.unwrap_or(Box::new(ListNode::new(0)));
            let l2_current_node = l2.unwrap_or(Box::new(ListNode::new(0)));
            let mut sum = l1_current_node.val + l2_current_node.val + carry;
            carry = sum / 10;
            sum = sum % 10;
            let node = Some(Box::new(ListNode::new(sum)));
            tail.next = node;
            tail = tail.next.as_mut().unwrap();
            l1 = l1_current_node.next;
            l2 = l2_current_node.next;
        }
        if carry > 0 {
            tail.next = Some(Box::new(ListNode::new(carry)));
        }
        head.next
    }
}

fn main() {
    let mut l1 = Some(Box::new(ListNode::new(8)));
    let next = Some(Box::new(ListNode::new(1)));
    l1.as_mut().unwrap().next = next;
    let mut l2 = Some(Box::new(ListNode::new(9)));
    let next = Some(Box::new(ListNode::new(9)));
    l2.as_mut().unwrap().next = next;
    let result = Solution::add_two_numbers(l1, l2);
    println!("{:?}", result);
}
