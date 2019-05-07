use super::*;

pub fn validate<T: PartialOrd>(tree: &AvlTreeNode<T>) -> bool {
    match tree {
        None => {}
        Some(root) => {
            if tree.balance_factor().abs() > 1 {
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
