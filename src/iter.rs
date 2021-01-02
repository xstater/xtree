use crate::Tree;

pub struct DfsIter<'a,T>{
    pub (in crate) stack : Vec<&'a Tree<T>>
}

impl<'a,T> DfsIter<'a,T> {
    pub fn depth(self) -> DfsDepthIter<'a,T>{
        DfsDepthIter {
            stack : vec![(0,self.stack.first().unwrap())]
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

pub struct DfsDepthIter<'a,T> {
    stack : Vec<(usize,&'a Tree<T>)>
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




