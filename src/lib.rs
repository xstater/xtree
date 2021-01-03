//! # XTree
//! XTree is a general purpose tree data structure.
//! ## Construct tree
//! ```
//! extern crate xtree;
//! use xtree::*;
//! let tree =
//!     tr!(1)
//!         / (tr!(2)
//!             / tr!(3)
//!             / tr!(4))
//!         / tr!(5);
//! ```
//! It will construct a tree like below
//! ```
//! //      1
//! //     / \
//! //   2    5
//! //  / \
//! // 3   4
//! ```
//! ## Depth-First iterate a tree
//! ```
//! for value in tree.dfs_iter(){
//!     print!("{} ",value);
//! }
//! ```
//! It will print ```1 2 3 4 5``` in console.
//! ## Breadth-First iterate a tree and Change the value
//! ```
//! for value in tree.bfs_iter_mut(){
//!     *value += 1;
//!     print!("{} ",value);
//! }
//! ```
//! It will print ```2 3 6 4 5``` in console.
//! ## Freely visit node with Cursor
//! ```
//! let mut cursor = tree.cursor();
//! ```
//! create a read-only cursor to root node.
//! ```
//! cursor.move_child(0);
//! ```
//! move this cursor to the first child node.
//! ```
//! println!("{}",cursor.current());
//! ```
//! get the value of which it pointing now.<br>
//! it will print ```2``` in console.
//! ## Advanced usage
//! [More exmples](https://github.com/xstater/xtree/tree/master/examples)
mod tree;
mod cursor;
mod df_iter;
mod bf_iter;

pub use tree::Tree;
pub use cursor::{Cursor,CursorMut};
pub use df_iter::*;
pub use bf_iter::*;

