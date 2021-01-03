use crate::cursor::{Cursor, CursorMut};
use std::ops::Div;
use crate::df_iter::{DfIter, DfIterMut};
use crate::bf_iter::{BfIter, BfIterMut};
use std::collections::VecDeque;

///The type representing a Tree
pub struct Tree<T> {
    pub (in crate) data : T,
    pub (in crate) children : Vec<Tree<T>>,
}

impl<T> Tree<T>{
    /// Create a tree only have one node with value
    /// # Example
    /// ```
    /// use xtree::Tree;
    /// let tree = Tree::new(3);
    /// ```
    /// !! Use tr! macro to instead
    pub fn new(data : T) -> Tree<T> {
        Tree{
            data,
            children : Vec::new()
        }
    }

    /// Add a child to root
    /// # Example
    /// ```
    /// let mut t1 = tr!(1);
    /// let t2 = tr!(2);
    /// t1.add_child(t2);
    /// ```
    /// !! Use '/' operator to instead
    pub fn add_child(&mut self,child : Tree<T>){
        self.children.push(child);
    }

    /// Get a immutable Cursor which points the root
    pub fn cursor(&self) -> Cursor<'_,T>{
        Cursor {
            root: &self,
            parents: vec![],
            now: &self,
        }
    }

    /// Get a mutable Cursor which points the root
    pub fn cursor_mut(&mut self) -> CursorMut<'_,T>{
        CursorMut{
            root: self as *mut _,
            parents: vec![],
            now: self as *mut _,
            _marker: Default::default()
        }
    }

    /// Get a immutable Depth-First iterator
    pub fn df_iter(&self) -> DfIter<'_,T> {
        DfIter {
            stack: vec![self]
        }
    }

    /// Get a mutable Depth-First iterator
    pub fn df_iter_mut(&mut self) -> DfIterMut<'_,T> {
        DfIterMut {
            stack: vec![self]
        }
    }

    /// Get a immutable Breadth-first iterator
    pub fn bf_iter(&self) -> BfIter<'_,T>{
        let mut v = VecDeque::new();
        v.push_back(self);
        BfIter {
            queue: v
        }
    }

    /// Get a mutable Breadth-First iterator
    pub fn bf_iter_mut(&mut self) -> BfIterMut<'_,T>{
        let mut v = VecDeque::new();
        v.push_back(self);
        BfIterMut {
            queue: v
        }
    }
}

impl<T> Div for Tree<T> {
    type Output = Tree<T>;

    fn div(mut self, rhs: Self) -> Self::Output {
        self.add_child(rhs);
        self
    }
}

/// A useful macro is used to build a tree
#[macro_export]
macro_rules! tr {
    ($value:expr) => {Tree::new($value)};
}

