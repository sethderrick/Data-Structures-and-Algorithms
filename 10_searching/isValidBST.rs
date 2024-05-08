use std::cell::RefCell;
use std::rc::Rc;

type TreeLink = Option<Rc<RefCell<TreeNode>>>;

struct TreeNode {
    val: i32,
    left: TreeLink,
    right: TreeLink,
}

impl TreeNode {
    fn new(val: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(TreeNode {
            val,
            left: None,
            right: None,
        }))
    }
}

/// Checks if a binary tree is a valid binary search tree.
fn is_valid_bst(root: &TreeLink) -> bool {
    traverse_in_order(root, i32::MIN, i32::MAX)
}

/// Helper function to perform the in-order traversal and validate the BST properties.
fn traverse_in_order(node: &TreeLink, min: i32, max: i32) -> bool {
    match node {
        Some(n) => {
            let n_borrow = n.borrow();
            if n_borrow.val < min || n_borrow.val > max {
                return false;
            }
            traverse_in_order(&n_borrow.left, min, n_borrow.val)
                && traverse_in_order(&n_borrow.right, n_borrow.val, max)
        }
        None => true,
    }
}

fn main() {
    // Example to test the function: Constructing a small BST.
    let root = TreeNode::new(2);
    let left = TreeNode::new(1);
    let right = TreeNode::new(3);

    {
        let mut root_borrow = root.borrow_mut();
        root_borrow.left = Some(left);
        root_borrow.right = Some(right);
    }

    println!("Is the tree a valid BST? {}", is_valid_bst(&Some(root)));
}
