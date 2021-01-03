use crate::tree::Tree;
use std::marker::PhantomData;

pub struct Cursor<'a,T> {
    pub (in crate) root : &'a Tree<T>,
    pub (in crate) parents : Vec<&'a Tree<T>>,
    pub (in crate) now : &'a Tree<T>,
}

impl<'a,T> Cursor<'a,T> {
    pub fn move_child(&mut self,at : usize){
        if at < self.children_count() {
            self.parents.push(self.now);
            self.now = self.now.children.get(at).unwrap();//no failure
        }else{
            panic!("Cursor::move_child(): index {} is out of children's length",at);
        }
    }

    pub fn move_parent(&mut self){
        if let Some(parent) = self.parents.pop(){
            self.now = parent;
        }
    }

    pub fn move_root(&mut self){
        self.now = self.root;
        self.parents.clear();
    }

    pub fn current(&self) -> &'a T {
        &self.now.data
    }

    pub fn children_count(&self) -> usize {
        self.now.children.len()
    }

    pub fn is_root(&self) -> bool {
        self.parents.is_empty()
    }
}


pub struct CursorMut<'a,T>{
    pub (in crate) root : *mut Tree<T>,
    pub (in crate) parents : Vec<*mut Tree<T>>,
    pub (in crate) now : *mut Tree<T>,
    pub (in crate) _marker : PhantomData<&'a T>
}

impl<'a,T> CursorMut<'a,T>{
    pub fn move_child(&mut self,at : usize){
        if at < self.children_count() {
            self.parents.push(self.now);
            self.now = unsafe{&mut *self.now}.children.get_mut(at).unwrap();//no failure
        }else{
            panic!("CursorMut::move_child(): index {} is out of children's length",at);
        }
    }

    pub fn move_parent(&mut self){
        if let Some(parent) = self.parents.pop(){
            self.now = parent;
        }
    }

    pub fn move_root(&mut self){
        self.now = self.root;
        self.parents.clear();
    }

    pub fn current(&self) -> &'a mut T {
        &mut unsafe{&mut *self.now}.data
    }

    pub fn children_count(&self) -> usize {
        unsafe{&*self.now}.children.len()
    }

    pub fn is_root(&self) -> bool {
        self.parents.is_empty()
    }

    ///root cannot be moved
    pub fn remove(self) -> Option<Tree<T>>{
        if self.is_root() {
            Option::None
        }else{
            let parent = unsafe{&mut **self.parents.last().unwrap()};
            let mut index = 0;
            for (i,ptr) in parent.children.iter().enumerate() {
                if (ptr as *const _) == self.now {
                    index = i;
                    break;
                }
            }
            Option::Some(parent.children.remove(index))
        }
    }

    pub fn add_child(&mut self,tree : Tree<T>){
        unsafe{&mut *self.now}.children.push(tree);
    }

}