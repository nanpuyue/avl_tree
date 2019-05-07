#![feature(bind_by_move_pattern_guards)]

use core::cmp::{max, Ordering};
use core::mem::swap;

pub use util::*;
use DeleteValue::*;
use InnerResult::*;

#[cfg(test)]
mod test;
mod util;

pub type AvlTreeNode<T> = Option<Box<TreeNode<T>>>;

#[derive(Clone, Debug)]
pub struct TreeNode<T: PartialOrd> {
    val: T,
    height: i32,
    left: AvlTreeNode<T>,
    right: AvlTreeNode<T>,
}

enum InnerResult {
    Left,     //操作在左子树完成
    Right,    //操作在右子树完成
    Unknown,  //树的平衡性未知
    Balanced, //树已平衡
}

#[derive(Debug)]
enum DeleteValue<T: PartialOrd> {
    Min,
    Max,
    Val(T),
    Del(AvlTreeNode<T>),
}

impl<T: PartialOrd> PartialEq<Box<TreeNode<T>>> for DeleteValue<T> {
    fn eq(&self, other: &Box<TreeNode<T>>) -> bool {
        match self {
            Min => other.left.is_none(),
            Max => other.right.is_none(),
            _ => false,
        }
    }
}

impl<T: PartialOrd> PartialEq<T> for DeleteValue<T> {
    fn eq(&self, other: &T) -> bool {
        match self {
            Val(v) => v == other,
            _ => false,
        }
    }
}

impl<T: PartialOrd> PartialOrd<T> for DeleteValue<T> {
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        match self {
            Min => Some(Ordering::Less),
            Max => Some(Ordering::Greater),
            Val(v) => v.partial_cmp(other),
            _ => None,
        }
    }
}

trait __AvlTree<T: PartialOrd>: Sized {
    fn rotate_ll(&mut self);
    fn rotate_rr(&mut self);
    fn rotate_lr(&mut self);
    fn rotate_rl(&mut self);
    fn update_height(&mut self);
    fn balance_factor(&self) -> i32;
    fn do_insert(&mut self, val: T) -> InnerResult;
    fn do_delete(&mut self, val: &mut DeleteValue<T>) -> InnerResult;
}

pub trait AvlTree<T: PartialOrd> {
    fn new(val: T) -> Self;
    fn height(&self) -> i32;
    fn insert(&mut self, val: T);
    fn delete(&mut self, val: T) -> Self;
}

impl<T: PartialOrd> __AvlTree<T> for AvlTreeNode<T> {
    fn rotate_ll(&mut self) {
        match self {
            None => unreachable!(),
            Some(root) => {
                let left = &mut root.left.take();
                match left {
                    None => unreachable!(),
                    Some(x) => {
                        swap(&mut root.left, &mut x.right);
                        self.update_height();
                        swap(self, &mut x.right);
                        swap(self, left);
                        self.update_height();
                    }
                }
            }
        }
    }

    fn rotate_rr(&mut self) {
        match self {
            None => unreachable!(),
            Some(root) => {
                let right = &mut root.right.take();
                match right {
                    None => unreachable!(),
                    Some(x) => {
                        swap(&mut root.right, &mut x.left);
                        self.update_height();
                        swap(self, &mut x.left);
                        swap(self, right);
                        self.update_height();
                    }
                }
            }
        }
    }

    fn rotate_lr(&mut self) {
        match self {
            None => unreachable!(),
            Some(root) => {
                root.left.rotate_rr();
                self.rotate_ll();
            }
        }
    }

    fn rotate_rl(&mut self) {
        match self {
            None => unreachable!(),
            Some(root) => {
                root.right.rotate_ll();
                self.rotate_rr();
            }
        }
    }

    fn update_height(&mut self) {
        match self {
            None => return,
            Some(x) => x.height = max(x.left.height(), x.right.height()) + 1,
        }
    }

    fn balance_factor(&self) -> i32 {
        match self {
            None => 0,
            Some(x) => x.left.height() - x.right.height(),
        }
    }

    fn do_insert(&mut self, val: T) -> InnerResult {
        match self {
            None => {
                *self = Self::new(val);
                Unknown
            }
            Some(root) => {
                let ret = {
                    if val < root.val {
                        match root.left.do_insert(val) {
                            Balanced => Balanced,
                            x if self.balance_factor() == 2 => {
                                match x {
                                    Left => self.rotate_ll(),
                                    Right => self.rotate_lr(),
                                    _ => unreachable!(),
                                }
                                Balanced
                            }
                            _ => Left,
                        }
                    } else {
                        match root.right.do_insert(val) {
                            Balanced => Balanced,
                            x if self.balance_factor() == -2 => {
                                match x {
                                    Left => self.rotate_rl(),
                                    Right => self.rotate_rr(),
                                    _ => unreachable!(),
                                }
                                Balanced
                            }
                            _ => Right,
                        }
                    }
                };
                self.update_height();
                ret
            }
        }
    }

    fn do_delete(&mut self, val: &mut DeleteValue<T>) -> InnerResult {
        match self {
            None => {
                *val = Del(None);
                Balanced
            }
            Some(root) => {
                let ret = {
                    if *val == root.val || *val == *root {
                        if root.left.is_some() {
                            if root.right.is_some() {
                                if root.left.height() > root.right.height() {
                                    let mut left_max = Max;
                                    root.left.do_delete(&mut left_max);
                                    match left_max {
                                        Del(Some(ref mut x)) => {
                                            swap(&mut root.val, &mut x.val);
                                            *val = left_max;
                                        }
                                        _ => unreachable!(),
                                    }
                                } else {
                                    let mut right_min = Min;
                                    root.right.do_delete(&mut right_min);
                                    match right_min {
                                        Del(Some(ref mut x)) => {
                                            swap(&mut root.val, &mut x.val);
                                            *val = right_min;
                                        }
                                        _ => unreachable!(),
                                    }
                                }
                            } else {
                                let mut left = root.left.take();
                                swap(self, &mut left);
                                *val = Del(left);
                            }
                        } else if root.right.is_some() {
                            let mut right = root.right.take();
                            swap(self, &mut right);
                            *val = Del(right);
                        } else {
                            let mut deleted = None;
                            swap(self, &mut deleted);
                            *val = Del(deleted);
                        }
                        self.update_height();
                        if self.balance_factor() == 0 {
                            Unknown
                        } else {
                            Balanced
                        }
                    } else if *val < root.val {
                        match root.left.do_delete(val) {
                            Balanced => Balanced,
                            Unknown => {
                                if self.balance_factor() == -2 {
                                    let right = self.as_ref().unwrap().right.as_ref().unwrap();
                                    if right.left.height() > right.right.height() {
                                        self.rotate_rl();
                                    } else {
                                        self.rotate_rr();
                                    }
                                }
                                if self.balance_factor() == 0 {
                                    Unknown
                                } else {
                                    Balanced
                                }
                            }
                            _ => unreachable!(),
                        }
                    } else {
                        match root.right.do_delete(val) {
                            Balanced => Balanced,
                            Unknown => {
                                if self.balance_factor() == 2 {
                                    let left = self.as_ref().unwrap().left.as_ref().unwrap();
                                    if left.left.height() >= left.right.height() {
                                        self.rotate_ll();
                                    } else {
                                        self.rotate_lr();
                                    }
                                }
                                if self.balance_factor() == 0 {
                                    Unknown
                                } else {
                                    Balanced
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                };
                self.update_height();
                ret
            }
        }
    }
}

impl<T: PartialOrd> AvlTree<T> for AvlTreeNode<T> {
    fn new(val: T) -> Self {
        Some(Box::new(TreeNode {
            val,
            height: 1,
            left: None,
            right: None,
        }))
    }

    fn height(&self) -> i32 {
        match self {
            None => 0,
            Some(x) => x.height,
        }
    }

    fn insert(&mut self, val: T) {
        self.do_insert(val);
    }

    fn delete(&mut self, val: T) -> Self {
        let mut val = Val(val);
        self.do_delete(&mut val);
        match val {
            Del(x) => x,
            _ => unreachable!(),
        }
    }
}
