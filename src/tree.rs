use crate::cursor::Cursor;
use std::ops::Div;
use crate::iter::DfsIter;

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
            child_itr: self.children.iter()
        }
    }

    pub fn dfs_iter(&self) -> DfsIter<'_,T> {
        DfsIter{
            stack: vec![&self]
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

