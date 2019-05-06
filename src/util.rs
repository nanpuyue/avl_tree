use super::*;

pub fn validate<T: PartialOrd>(tree: &AvlTreeNode<T>) -> bool {
    match tree {
        None => {}
        Some(root) => {
            if (&root.left.height() - &root.right.height()).abs() > 1 {
                return false;
            }
            if let Some(x) = &root.left {
                if !(x.val < root.val && validate(&root.left)) {
                    return false;
                }
            }
            if let Some(x) = &root.right {
                if !(x.val > root.val && validate(&root.right)) {
                    return false;
                }
            }
        }
    }
    true
}
