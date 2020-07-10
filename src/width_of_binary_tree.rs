pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::rc::Rc;
use std::cell::RefCell;


pub fn run () -> i32 {
    
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if !root.is_some() {return 0;}
        let root = (*root.unwrap()).into_inner();
        let mut max = 1;
        let mut row = vec![(&root, 0)];

        while row.len() != 0 {
            let mut new_row: Vec<(&TreeNode, i32)> = Vec::new();
            let base = row[0].1 * 2;
            for node in &row {
                if let Some(left) = node.0.left {
                    let left = (*left).into_inner();
                    new_row.push((&left, node.1 * 2 - base));
                }
                if let Some(right) = node.0.right {
                    let right = (*right).into_inner();
                    new_row.push((&right, node.1 * 2 + 1 - base));
                }
                row = new_row;
                if row.len() > 1 {
                    let row_max = row[row.len() - 1].1 - row[0].1 + 1;
                    if row_max > max { max = row_max;}
                }
            }
        }
        max        
    }

    width_of_binary_tree(None)
}