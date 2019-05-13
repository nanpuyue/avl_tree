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
    Left,     //在左子树完成插入
    Right,    //在右子树完成插入
    Unknown,  //树的平衡性未知
    Balanced, //树已确定平衡
}

enum DeleteValue<T: PartialOrd> {
    Min,                 //匹配最小节点
    Max,                 //匹配最大节点
    Val(T),              //匹配给定值
    Del(AvlTreeNode<T>), //返回被删除节点
}

impl<T: PartialOrd> PartialEq<Box<TreeNode<T>>> for DeleteValue<T> {
    fn eq(&self, other: &Box<TreeNode<T>>) -> bool {
        match self {
            Min => other.left.is_none(),
            Max => other.right.is_none(),
            Val(v) => v == &other.val,
            _ => false,
        }
    }
}

impl<T: PartialOrd> PartialOrd<Box<TreeNode<T>>> for DeleteValue<T> {
    fn partial_cmp(&self, other: &Box<TreeNode<T>>) -> Option<Ordering> {
        match self {
            Min => Some(Ordering::Less),
            Max => Some(Ordering::Greater),
            Val(v) => v.partial_cmp(&other.val),
            _ => None,
        }
    }
}

trait __AvlTree<T: PartialOrd> {
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
            Some(root) => {
                let left = &mut root.left.take();
                match left {
                    Some(x) => {
                        swap(&mut root.left, &mut x.right);
                        self.update_height();
                        swap(self, &mut x.right);
                        swap(self, left);
                        self.update_height();
                    }
                    None => unreachable!(),
                }
            }
            None => unreachable!(),
        }
    }

    fn rotate_rr(&mut self) {
        match self {
            Some(root) => {
                let right = &mut root.right.take();
                match right {
                    Some(x) => {
                        swap(&mut root.right, &mut x.left);
                        self.update_height();
                        swap(self, &mut x.left);
                        swap(self, right);
                        self.update_height();
                    }
                    None => unreachable!(),
                }
            }
            None => unreachable!(),
        }
    }

    fn rotate_lr(&mut self) {
        match self {
            Some(root) => {
                root.left.rotate_rr();
                self.rotate_ll();
            }
            None => unreachable!(),
        }
    }

    fn rotate_rl(&mut self) {
        match self {
            Some(root) => {
                root.right.rotate_ll();
                self.rotate_rr();
            }
            None => unreachable!(),
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
            //直接插入新节点
            None => {
                *self = Self::new(val);
                Unknown
            }
            //递归插入
            Some(root) => {
                //重复数据
                if val == root.val {
                    return Balanced;
                //进入左子树递归插入
                } else if val < root.val {
                    match root.left.do_insert(val) {
                        Balanced => return Balanced,
                        x if self.balance_factor() == 2 => {
                            match x {
                                Left => self.rotate_ll(),
                                Right => self.rotate_lr(),
                                _ => unreachable!(),
                            }
                            return Balanced;
                        }
                        _ => {
                            self.update_height();
                            Left
                        }
                    }
                //进入右子树递归插入
                } else {
                    match root.right.do_insert(val) {
                        Balanced => return Balanced,
                        x if self.balance_factor() == -2 => {
                            match x {
                                Left => self.rotate_rl(),
                                Right => self.rotate_rr(),
                                _ => unreachable!(),
                            }
                            return Balanced;
                        }
                        _ => {
                            self.update_height();
                            Right
                        }
                    }
                }
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
                let height = root.height;

                //删除当前节点
                if val == root {
                    if root.left.is_some() {
                        //左右子树均非空
                        if root.right.is_some() {
                            if root.left.height() > root.right.height() {
                                *val = Max;
                                root.left.do_delete(val);
                                match val {
                                    Del(Some(x)) => {
                                        swap(&mut root.val, &mut x.val);
                                    }
                                    _ => unreachable!(),
                                }
                            } else {
                                *val = Min;
                                root.right.do_delete(val);
                                match val {
                                    Del(Some(x)) => {
                                        swap(&mut root.val, &mut x.val);
                                    }
                                    _ => unreachable!(),
                                }
                            }
                        //左子树非空，右子树为空
                        } else {
                            let mut left = root.left.take();
                            swap(self, &mut left);
                            *val = Del(left);
                        }
                    //左子树为空，右子树非空或为空
                    } else {
                        let mut right = root.right.take();
                        swap(self, &mut right);
                        *val = Del(right);
                    }
                    self.update_height();
                //进入左子树递归删除
                } else if val < root {
                    match root.left.do_delete(val) {
                        Balanced => return Balanced,
                        Unknown => {
                            if self.balance_factor() == -2 {
                                let right = self.as_ref().unwrap().right.as_ref().unwrap();
                                if right.left.height() > right.right.height() {
                                    self.rotate_rl();
                                } else {
                                    self.rotate_rr();
                                }
                            } else {
                                self.update_height();
                            }
                        }
                        _ => unreachable!(),
                    }
                //进入右子树递归删除
                } else {
                    match root.right.do_delete(val) {
                        Balanced => return Balanced,
                        Unknown => {
                            if self.balance_factor() == 2 {
                                let left = self.as_ref().unwrap().left.as_ref().unwrap();
                                if left.left.height() >= left.right.height() {
                                    self.rotate_ll();
                                } else {
                                    self.rotate_lr();
                                }
                            } else {
                                self.update_height();
                            }
                        }
                        _ => unreachable!(),
                    }
                }

                if self.height() == height {
                    Balanced
                } else {
                    Unknown
                }
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
