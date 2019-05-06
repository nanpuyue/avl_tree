#![feature(bind_by_move_pattern_guards)]

use core::cmp::max;
use core::mem::swap;

use InsertResult::*;

type AvlTreeNode<T> = Option<Box<TreeNode<T>>>;

#[derive(Clone, Debug)]
struct TreeNode<T: PartialOrd> {
    val: T,
    height: i32,
    left: AvlTreeNode<T>,
    right: AvlTreeNode<T>,
}

#[derive(Debug)]
enum InsertResult {
    Left,
    Right,
    True,
    False,
}

trait AvlTree<T: PartialOrd> {
    fn new(val: T) -> Self;
    fn height(&self) -> i32;
    fn update_height(&mut self);
    fn rotate_ll(&mut self);
    fn rotate_rr(&mut self);
    fn rotate_lr(&mut self);
    fn rotate_rl(&mut self);
    fn insert(&mut self, val: T) -> InsertResult;
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

    fn update_height(&mut self) {
        match self {
            None => return,
            Some(x) => x.height = max(x.left.height(), x.right.height()) + 1,
        }
    }

    fn rotate_ll(&mut self) {
        match self {
            None => return,
            Some(root) => {
                let left = &mut root.left.take();
                match left {
                    None => unreachable!(),
                    Some(x) => {
                        root.left = x.right.take();
                        self.update_height();
                        swap(&mut x.right, self);
                        swap(left, self);
                        self.update_height();
                    }
                }
            }
        }
    }

    fn rotate_rr(&mut self) {
        match self {
            None => return,
            Some(root) => {
                let right = &mut root.right.take();
                match right {
                    None => unreachable!(),
                    Some(x) => {
                        root.right = x.left.take();
                        self.update_height();
                        swap(&mut x.left, self);
                        swap(right, self);
                        self.update_height();
                    }
                }
            }
        }
    }

    fn rotate_lr(&mut self) {
        match self {
            None => return,
            Some(root) => {
                root.left.rotate_rr();
                self.rotate_ll();
            }
        }
    }

    fn rotate_rl(&mut self) {
        match self {
            None => return,
            Some(root) => {
                root.right.rotate_ll();
                self.rotate_rr();
            }
        }
    }

    fn insert(&mut self, val: T) -> InsertResult {
        match self {
            None => {
                *self = Self::new(val);
                True
            }
            Some(root) => {
                let ret = {
                    if val < root.val {
                        match root.left.insert(val) {
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
                        match root.right.insert(val) {
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

fn main() {
    let mut avl_tree = None;
    for i in &vec![50,16,7,43,21,14,30,11,42,17,10,49,33,48,46,1,45,9,6,15] {
        avl_tree.insert(*i);
    }
    dbg!(&avl_tree);
}
