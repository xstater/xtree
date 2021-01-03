use crate::Tree;
use std::collections::VecDeque;

/// The immutable Breadth-First Iterator.
pub struct BfIter<'a,T>{
    pub (in crate) queue : VecDeque<&'a Tree<T>>
}

/// The mutable Breadth-First Iterator.
pub struct BfIterMut<'a,T>{
    pub (in crate) queue : VecDeque<&'a mut Tree<T>>
}

impl<'a,T> BfIter<'a,T> {
    /// convert to a BfDepthIter
    pub fn depth(self) -> BfDepthIter<'a,T>{
        let mut s = self;
        let mut v = VecDeque::new();
        v.push_back((0,s.queue.pop_front().unwrap()));
        BfDepthIter {
            queue : v
        }
    }
}

impl<'a,T> BfIterMut<'a,T>{
    /// convert to a BfDepthIterMut
    pub fn depth(self) -> BfDepthIterMut<'a,T> {
        let mut s = self;
        let mut v = VecDeque::new();
        v.push_back((0,s.queue.pop_front().unwrap()));
        BfDepthIterMut {
            queue : v
        }
    }
}

impl<'a,T> Iterator for BfIter<'a,T>{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(now) = self.queue.pop_front() {
            for child in now.children.iter() {
                self.queue.push_back(child);
            }
            Some(&now.data)
        }else{
            Option::None
        }
    }
}

impl<'a,T> Iterator for BfIterMut<'a,T>{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(now) = self.queue.pop_front() {
            for child in now.children.iter_mut() {
                self.queue.push_back(child);
            }
            Some(&mut now.data)
        }else{
            Option::None
        }
    }
}

/// The immutable Breadth-First Iterator with Depth information.
pub struct BfDepthIter<'a,T> {
    queue : VecDeque<(usize,&'a Tree<T>)>
}

/// The mutable Breadth-First Iterator with Depth information.
pub struct BfDepthIterMut<'a,T> {
    queue : VecDeque<(usize,&'a mut Tree<T>)>
}

impl<'a,T> Iterator for BfDepthIter<'a,T> {
    type Item = (usize,&'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((dpth,now)) = self.queue.pop_front() {
            for child in now.children.iter() {
                self.queue.push_back((dpth + 1,child));
            }
            Some((dpth,&now.data))
        }else{
            Option::None
        }
    }
}

impl<'a,T> Iterator for BfDepthIterMut<'a,T> {
    type Item = (usize,&'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((dpth,now)) = self.queue.pop_front() {
            for child in now.children.iter_mut() {
                self.queue.push_back((dpth + 1,child));
            }
            Some((dpth,&mut now.data))
        }else{
            Option::None
        }
    }
}


