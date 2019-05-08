use core::fmt::Display;

use super::*;

pub fn validate<T: PartialOrd>(tree: &AvlTreeNode<T>) -> bool {
    if let Some(root) = tree {
        if root.height != max(root.left.height(), root.right.height()) + 1 {
            return false;
        }
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
    true
}

pub fn print_dot<T: PartialOrd + Display>(tree: &AvlTreeNode<T>, parent: &AvlTreeNode<T>) {
    if parent.is_none() {
        println!("digraph {{");
    }
    if let Some(root) = tree {
        if let Some(x) = &root.left {
            println!("    {}->{}", root.val, x.val);
            print_dot(&root.left, tree);
        }
        if let Some(x) = &root.right {
            println!("    {}->{}", root.val, x.val);
            print_dot(&root.right, tree);
        }
    }
    if parent.is_none() {
        println!("}}");
    }
}
