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
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut one = 0;
        let mut p1 = l1;
        let mut p2 = l2;
        
        let mut head = Box::new(ListNode::new(0));
        let mut ptr = &mut head;

        while one == 1 || p1 != None || p2 != None {
            let mut num = one;
            match p1 {
                Some(node) => {
                    num += node.val;
                    p1 = node.next;
                },
                None => {},
            }
            match p2 {
                Some(node) => {
                    num += node.val;
                    p2 = node.next;
                }
                None => {},
            }
            
            if num >= 10 {
                num -= 10;
                one = 1;
            } else {
                one = 0;
            }
            
            ptr.next = Some(Box::new(ListNode::new(num)));
            ptr = ptr.next.as_mut().unwrap();
        }
        
        return head.next;
    }
}
