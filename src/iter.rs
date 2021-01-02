use crate::Tree;

pub struct DfsIter<'a,T>{
    pub (in crate) stack : Vec<&'a Tree<T>>
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