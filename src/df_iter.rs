use crate::Tree;

/// The immutable Depth-First Iterator.
pub struct DfIter<'a,T>{
    pub (in crate) stack : Vec<&'a Tree<T>>
}

/// The mutable Depth-First Iterator.
pub struct DfIterMut<'a,T>{
    pub (in crate) stack : Vec<&'a mut Tree<T>>
}

impl<'a,T> DfIter<'a,T> {
    /// convert to a DfDepthIter
    pub fn depth(self) -> DfDepthIter<'a,T>{
        let mut s = self;
        DfDepthIter {
            stack : vec![(0,s.stack.pop().unwrap())]
        }
    }
}

impl<'a,T> DfIterMut<'a,T>{
    /// convert to a DfDepthIterMut
    pub fn depth(self) -> DfDepthIterMut<'a,T> {
        let mut s = self;
        DfDepthIterMut {
            stack: vec![(0,s.stack.pop().unwrap())],
        }
    }
}

impl<'a,T> Iterator for DfIter<'a,T>{
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

impl<'a,T> Iterator for DfIterMut<'a,T>{
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

/// The immutable Depth-First Iterator with Depth information.
pub struct DfDepthIter<'a,T> {
    stack : Vec<(usize,&'a Tree<T>)>
}

/// The mutable Breadth-First Iterator with Depth information.
pub struct DfDepthIterMut<'a,T> {
    stack : Vec<(usize,&'a mut Tree<T>)>
}

impl<'a,T> Iterator for DfDepthIter<'a,T> {
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

impl<'a,T> Iterator for DfDepthIterMut<'a,T> {
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


