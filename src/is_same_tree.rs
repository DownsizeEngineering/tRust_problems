use std::rc::Rc;
use std::cell::RefCell;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}


pub fn run() -> bool {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if p == None && q == None {return true;}
        else {
            let p = p.unwrap().get_ref();
            let q = q.unwrap().get_ref();

            if p.val == q.val
            && is_same_tree(p.left, q.left)
            && is_same_tree(p.right, q.right) {
                return true;
            }
        }
        return false;
    }

    is_same_tree(None, None)
}