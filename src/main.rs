use std::cell::RefCell;
use std::rc::Rc;

type AvlTreeNode<T> = Option<Rc<RefCell<TreeNode<T>>>>;

#[derive(Clone, Debug)]
struct TreeNode<T: PartialOrd> {
    val: T,
    height: usize,
    left: AvlTreeNode<T>,
    right: AvlTreeNode<T>,
}

trait AvlTree<T: PartialOrd> {
    fn new(val: T) -> Self;
    fn insert(&mut self, val: T) -> bool;
}

impl<T: PartialOrd> AvlTree<T> for AvlTreeNode<T> {
    fn new(val: T) -> Self {
        Some(Rc::new(RefCell::new(TreeNode {
            val,
            height: 0,
            left: None,
            right: None,
        })))
    }

    fn insert(&mut self, val: T) -> bool {
        match self {
            None => {
                *self = Self::new(val);
                false
            }
            Some(root) => {
                let mut root = root.borrow_mut();
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
    for i in &[3, 4, 1, 6, 3, 8, 9, 5] {
        avl_tree.insert(*i);
    }
    dbg!(avl_tree);
}
