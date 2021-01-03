use crate::Tree;
use std::collections::VecDeque;

pub struct BfsIter<'a,T>{
    pub (in crate) queue : VecDeque<&'a Tree<T>>
}

pub struct BfsIterMut<'a,T>{
    pub (in crate) queue : VecDeque<&'a mut Tree<T>>
}

impl<'a,T> BfsIter<'a,T> {
    pub fn depth(self) -> BfsDepthIter<'a,T>{
        let mut s = self;
        let mut v = VecDeque::new();
        v.push_back((0,s.queue.pop_front().unwrap()));
        BfsDepthIter {
            queue : v
        }
    }
}

impl<'a,T> BfsIterMut<'a,T>{
    pub fn depth(self) -> BfsDepthIterMut<'a,T> {
        let mut s = self;
        let mut v = VecDeque::new();
        v.push_back((0,s.queue.pop_front().unwrap()));
        BfsDepthIterMut {
            queue : v
        }
    }
}

impl<'a,T> Iterator for BfsIter<'a,T>{
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

impl<'a,T> Iterator for BfsIterMut<'a,T>{
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

pub struct BfsDepthIter<'a,T> {
    queue : VecDeque<(usize,&'a Tree<T>)>
}

pub struct BfsDepthIterMut<'a,T> {
    queue : VecDeque<(usize,&'a mut Tree<T>)>
}

impl<'a,T> Iterator for BfsDepthIter<'a,T> {
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

impl<'a,T> Iterator for BfsDepthIterMut<'a,T> {
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


