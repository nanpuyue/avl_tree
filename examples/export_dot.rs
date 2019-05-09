//! # Usage
//!
//! ```bash
//! cargo run --example export_dot > tree.dot
//! dot tree.dot -Tsvg -otree.svg
//! ```
//!
//! Or
//!
//! ```bash
//! cargo run --example export_dot | dot -Tsvg -otree.svg
//! ```

use avl_tree::*;

fn main() {
    let mut avl_tree = None;
    for i in vec![
        36, 24, 39, 20, 3, 44, 43, 6, 10, 48, 2, 27, 15, 28, 49, 47, 9, 21, 46, 29, 25, 1, 22, 34,
        16, 38, 19, 32, 40, 31, 30, 50, 14, 42, 13, 8, 45, 5, 41, 26, 33, 37, 4, 12, 18, 35, 17, 7,
        11, 23,
    ]
    .into_iter()
    {
        avl_tree.insert(i);
    }

    print_dot(&avl_tree);
}
