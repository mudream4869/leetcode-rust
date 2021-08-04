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
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut ptr1 = &head;
        let mut ptr2 = &head;
        while *ptr2 != None && ptr2.as_ref().unwrap().next != None {
            ptr1 = &(ptr1.as_ref().unwrap().next);
            ptr2 = &(ptr2.as_ref().unwrap().next.as_ref().unwrap().next);
        }
        ptr1.clone()
    }
}
