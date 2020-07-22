use std::rc::Rc;
use std::cell::RefCell;

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

pub fn run() -> Vec<Vec<i32>> {
    let e = Rc::new(RefCell::new(TreeNode::new(7)));           //    3
    let d = Rc::new(RefCell::new(TreeNode::new(15)));          //   / \
    let c = TreeNode {val: 20, left: Some(d), right: Some(e)}; //  9  20
    let c = Rc::new(RefCell::new(c));                          //    /  \
    let b = Rc::new(RefCell::new(TreeNode::new(9)));           //   15   7
    let a = TreeNode {val: 3, left: Some(b), right: Some(c)};
    let a = Rc::new(RefCell::new(a));


    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut output = Vec::new();
        if root.is_none() { return output; }

        fn recurse(
            output: &mut Vec<Vec<i32>>, 
            node: Rc<RefCell<TreeNode>>,
            height: usize
        ) {
            if output.len() <= height { output.push(Vec::new()); }
            let node = node.borrow();
            output[height].push(node.val);
            if let Some(left) = node.left.clone() {
                recurse(output, left, height + 1);
            }
            if let Some(right) = node.right.clone() {
                recurse(output, right, height + 1);
            }
        }

        recurse(&mut output, root.unwrap(), 0);

        fn reverse_even_rows<T: Clone>(mut input: Vec<Vec<T>>) -> Vec<Vec<T>> {
            let mut flag = false;
            
            for i in 0..input.len() {
                if flag {
                    input[i] = input[i].iter().rev().cloned().collect();
                }
                flag = !flag;
            }

            input
        }
        reverse_even_rows(output)
    }

    zigzag_level_order(Some(a))
}