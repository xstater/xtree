use crate::tree::Tree;
use std::slice::Iter;

pub struct Cursor<'a,T> {
    ///the root node
    pub (in crate) root : &'a Tree<T>,
    ///the parents list
    pub (in crate) parents : Vec<&'a Tree<T>>,
    ///now
    pub (in crate) now : &'a Tree<T>,
    ///children iterator
    pub (in crate) child_itr : Iter<'a,Tree<T>>
}

impl<'a,T> Cursor<'a,T> {
    pub fn move_child(&mut self) -> Option<&'a T> {
        if let Some(child) = self.child_itr.next(){
            self.parents.push(self.now);
            self.now = child;
            Option::Some(&self.now.data)
        }else{
            Option::None
        }
    }

    pub fn move_child_next(&mut self){
        self.child_itr.next();
    }

    pub fn move_parent(&mut self) -> Option<&'a T> {
        if let Some(parent) = self.parents.pop(){
            self.now = parent;
            self.child_itr = self.now.children.iter();
            Option::Some(&self.now.data)
        }else{
            Option::None
        }
    }

    pub fn move_root(&mut self) -> &'a T {
        self.now = self.root;
        self.parents.clear();
        self.child_itr = self.now.children.iter();
        &self.now.data
    }

    pub fn current(&self) -> &'a T {
        &self.now.data
    }
}
