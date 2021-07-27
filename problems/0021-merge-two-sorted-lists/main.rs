// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut p1 = l1;
        let mut p2 = l2;
        let mut head = Box::new(ListNode::new(0));
        let mut ptr = &mut head;
        loop {
            match (p1, p2) {
                (Some(mut n1), Some(mut n2)) => {
                    if n1.val > n2.val {
                        p1 = Some(n1);
                        p2 = n2.next.take();
                        ptr.next = Some(n2);
                    } else {
                        p2 = Some(n2);
                        p1 = n1.next.take();
                        ptr.next = Some(n1);
                    }
                },
                (Some(n1), None) => {
                    ptr.next = Some(n1);
                    break;
                },
                (None, Some(n2)) => {
                    ptr.next = Some(n2);
                    break;
                },
                (None, None) => {
                    break;
                },
            }
            ptr = ptr.next.as_mut().unwrap();
        }
        return head.next;
    }
}
