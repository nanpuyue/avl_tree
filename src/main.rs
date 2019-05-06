use core::cmp::max;
use core::mem::swap;

type AvlTreeNode<T> = Option<Box<TreeNode<T>>>;

#[derive(Clone, Debug)]
struct TreeNode<T: PartialOrd> {
    val: T,
    height: usize,
    left: AvlTreeNode<T>,
    right: AvlTreeNode<T>,
}

trait AvlTree<T: PartialOrd> {
    fn new(val: T) -> Self;
    fn height(&self) -> usize;
    fn update_height(&mut self);
    fn rotate_ll(&mut self);
    fn rotate_rr(&mut self);
    fn rotate_lr(&mut self);
    fn rotate_rl(&mut self);
    fn insert(&mut self, val: T) -> bool;
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

    fn height(&self) -> usize {
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

    fn insert(&mut self, val: T) -> bool {
        match self {
            None => {
                *self = Self::new(val);
                false
            }
            Some(root) => {
                let mut ret = root.left.is_none() && root.right.is_none();
                ret |= {
                    if val < root.val {
                        root.left.insert(val)
                    } else {
                        root.right.insert(val)
                    }
                };
                if ret {
                    root.height += 1;
                }
                ret
            }
        }
    }
}

fn main() {
    let mut avl_tree = None;
    for i in &[5, 4, 3, 6, 7, 8] {
        avl_tree.insert(*i);
    }
    avl_tree.rotate_rr();
    dbg!(&avl_tree);
}
