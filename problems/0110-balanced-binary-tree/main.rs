// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn height_(root: Option<Rc<RefCell<TreeNode>>>) -> Result<i32, bool> {
        match root {
            None => {
                return Ok(0);
            },
            Some(node) => {
                let nb = node.borrow();
                let h1 = Self::height_(nb.left.clone())?;
                let h2 = Self::height_(nb.right.clone())?;
                if (h1 - h2).abs() <= 1 {
                    return Ok(std::cmp::max(h1, h2) + 1);
                } else {
                    return Err(false);
                }
            },
        }
    }
    
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match Self::height_(root) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
