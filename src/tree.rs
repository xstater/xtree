use crate::cursor::{Cursor, CursorMut};
use std::ops::Div;
use crate::dfs_iter::{DfsIter, DfsIterMut};
use crate::bfs_iter::{BfsIter, BfsIterMut};
use std::collections::VecDeque;

pub struct Tree<T> {
    pub (in crate) data : T,
    pub (in crate) children : Vec<Tree<T>>,
}

impl<T> Tree<T>{
    pub fn new(data : T) -> Tree<T> {
        Tree{
            data,
            children : Vec::new()
        }
    }

    pub fn add_child(&mut self,child : Tree<T>){
        self.children.push(child);
    }

    pub fn cursor(&self) -> Cursor<'_,T>{
        Cursor {
            root: &self,
            parents: vec![],
            now: &self,
        }
    }

    pub fn cursor_mut(&mut self) -> CursorMut<'_,T>{
        CursorMut{
            root: self as *mut _,
            parents: vec![],
            now: self as *mut _,
            _marker: Default::default()
        }
    }

    pub fn dfs_iter(&self) -> DfsIter<'_,T> {
        DfsIter{
            stack: vec![self]
        }
    }

    pub fn dfs_iter_mut(&mut self) -> DfsIterMut<'_,T> {
        DfsIterMut{
            stack: vec![self]
        }
    }

    pub fn bfs_iter(&self) -> BfsIter<'_,T>{
        let mut v = VecDeque::new();
        v.push_back(self);
        BfsIter{
            queue: v
        }
    }
    pub fn bfs_iter_mut(&mut self) -> BfsIterMut<'_,T>{
        let mut v = VecDeque::new();
        v.push_back(self);
        BfsIterMut{
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

#[macro_export]
macro_rules! tr {
    ($value:expr) => {Tree::new($value)};
}

