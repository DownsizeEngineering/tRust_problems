use std::boxed::Box;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Box<TreeNode>>,
  pub right: Option<Box<TreeNode>>,
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
    let mut tree = TreeNode::new(3);
    tree.left = Some(Box::new(TreeNode::new(9)));

    let mut right = TreeNode::new(20);
    right.left = Some(Box::new(TreeNode::new(15)));
    right.right = Some(Box::new(TreeNode::new(7)));

    tree.right = Some(Box::new(right));
    level_order_bottom(&tree)
}

pub fn level_order_bottom(root: &TreeNode) 
-> Vec<Vec<i32>> {
        let mut output: Vec<Vec<i32>> = Vec::new();

        fn recurse (node: &TreeNode, output: &mut Vec<Vec<i32>>, depth: usize) {
            if output.len() <= depth {output.push(Vec::new())}

            if let Some(x) = &node.left {
                recurse(x, output, depth + 1);
            }
            
            output[depth].push(node.val);

            if let Some(x) = &node.right {
                recurse(x, output, depth + 1);
            }
        }

        recurse(root, &mut output, 0);
        output.into_iter().rev().collect()
}