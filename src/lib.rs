#![feature(bind_by_move_pattern_guards)]

use core::cmp::max;
use core::mem::swap;

pub use util::*;
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
    Left,  //操作在左子树完成
    Right, //操作在右子树完成
    True,  //本次调用直接完成了操作
    False, //树已平衡，无需再进行旋转操作
}

trait __AvlTree<T: PartialOrd> {
    fn rotate_ll(&mut self);
    fn rotate_rr(&mut self);
    fn rotate_lr(&mut self);
    fn rotate_rl(&mut self);
    fn update_height(&mut self);
    fn do_insert(&mut self, val: T) -> InnerResult;
}

pub trait AvlTree<T: PartialOrd> {
    fn new(val: T) -> Self;
    fn height(&self) -> i32;
    fn insert(&mut self, val: T);
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

    fn do_insert(&mut self, val: T) -> InnerResult {
        match self {
            None => {
                *self = Self::new(val);
                True
            }
            Some(root) => {
                let ret = {
                    if val < root.val {
                        match root.left.do_insert(val) {
                            False => False,
                            x if root.left.height() - root.right.height() == 2 => {
                                match x {
                                    Left => self.rotate_ll(),
                                    Right => self.rotate_lr(),
                                    _ => unreachable!(),
                                }
                                False
                            }
                            _ => Left,
                        }
                    } else {
                        match root.right.do_insert(val) {
                            False => False,
                            x if root.right.height() - root.left.height() == 2 => {
                                match x {
                                    Left => self.rotate_rl(),
                                    Right => self.rotate_rr(),
                                    _ => unreachable!(),
                                }
                                False
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
}
