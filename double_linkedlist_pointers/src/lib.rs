use std::boxed::Box;
use std::fmt::Debug;

#[allow(dead_code)]

pub struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
    prev: Option<Box<Node<T>>>,
}

pub struct deque<T> {
    begin: Option<Box<Node<T>>>,
    end: Option<Box<Node<T>>>,
}

impl<T> deque <T> 
where
    T: Debug
{
    pub fn new() -> Self{
        Self{
            begin: None,
            end: None,
        }
    }

    pub fn push_front(&mut self, value: T){
        todo!()
    }

    pub fn push_back(&mut self, value: T){
        todo!()
    }

    fn debug_dump(&self){
        let mut iter = &self.begin;
        while let Some(node) = iter{
            print!("{:?}", node.value);
            iter = &node.next;
        }
        println!("")
    }
}
fn main() {
    let mut xs = deque::<String>::new();
    xs.push_front("a".to_string());
    xs.push_back("a".to_string());
    xs.push_front("a".to_string());
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        todo!()
    }
}
