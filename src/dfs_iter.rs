use crate::Tree;

pub struct DfsIter<'a,T>{
    pub (in crate) stack : Vec<&'a Tree<T>>
}

pub struct DfsIterMut<'a,T>{
    pub (in crate) stack : Vec<&'a mut Tree<T>>
}

impl<'a,T> DfsIter<'a,T> {
    pub fn depth(self) -> DfsDepthIter<'a,T>{
        let mut s = self;
        DfsDepthIter {
            stack : vec![(0,s.stack.pop().unwrap())]
        }
    }
}

impl<'a,T> DfsIterMut<'a,T>{
    pub fn depth(self) -> DfsDepthIterMut<'a,T> {
        let mut s = self;
        DfsDepthIterMut{
            stack: vec![(0,s.stack.pop().unwrap())],
        }
    }
}

impl<'a,T> Iterator for DfsIter<'a,T>{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(top) = self.stack.pop() {
            for child in top.children.iter().rev() {
                self.stack.push(child);
            }
            Some(&top.data)
        }else{
            Option::None
        }
    }
}

impl<'a,T> Iterator for DfsIterMut<'a,T>{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(top) = self.stack.pop() {
            for child in top.children.iter_mut().rev() {
                self.stack.push(child);
            }
            Some(&mut top.data)
        }else{
            Option::None
        }
    }
}

pub struct DfsDepthIter<'a,T> {
    stack : Vec<(usize,&'a Tree<T>)>
}

pub struct DfsDepthIterMut<'a,T> {
    stack : Vec<(usize,&'a mut Tree<T>)>
}

impl<'a,T> Iterator for DfsDepthIter<'a,T> {
    type Item = (usize,&'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((dpth,top)) = self.stack.pop() {
            for child in top.children.iter().rev() {
                self.stack.push((dpth + 1,child));
            }
            Some((dpth,&top.data))
        }else{
            Option::None
        }
    }
}

impl<'a,T> Iterator for DfsDepthIterMut<'a,T> {
    type Item = (usize,&'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((dpth,top)) = self.stack.pop() {
            for child in top.children.iter_mut().rev() {
                self.stack.push((dpth + 1,child));
            }
            Some((dpth,&mut top.data))
        }else{
            Option::None
        }
    }
}


