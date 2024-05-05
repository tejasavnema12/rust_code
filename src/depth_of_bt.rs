use std::io;
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
            right: None,
        }
    }
}

fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        Some(node) => {
            let node_ref = node.borrow();
            let left_depth = max_depth(node_ref.left.clone());
            let right_depth = max_depth(node_ref.right.clone());
            1 + left_depth.max(right_depth)
        }
        None => 0,
    }
}

fn main() {
    println!("Enter the values for the binary tree nodes separated by spaces (use -1 for null nodes):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let values: Vec<i32> = input.trim().split_whitespace()
                                    .map(|x| x.parse().expect("Invalid input"))
                                    .collect();

    let mut index = 0;
    let root = TreeNode::new(values[index]);
    index += 1;

    let root_cell = Rc::new(RefCell::new(root));
    let mut stack = vec![(Some(root_cell.clone()), 0)];

    while index < values.len() && !stack.is_empty() {
        let (parent, level) = stack.pop().unwrap();
        if let Some(p) = parent {
            let mut node = p.borrow_mut();
            if values[index] != -1 {
                node.left = Some(Rc::new(RefCell::new(TreeNode::new(values[index]))));
                stack.push((node.left.clone(), level + 1));
            }
            index += 1;
            if index < values.len() && values[index] != -1 {
                node.right = Some(Rc::new(RefCell::new(TreeNode::new(values[index]))));
                stack.push((node.right.clone(), level + 1));
            }
            index += 1;
        }
    }

    let max_depth = max_depth(Some(root_cell));
    println!("Max Depth: {}", max_depth);
}
