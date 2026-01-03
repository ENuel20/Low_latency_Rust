pub fn flatten<I>(iter: I) -> Flatten<I::IntoIter>
where
    I : IntoIterator,
    I::Item:IntoIterator,
{
    Flatten::new(iter.into_iter())
}

pub struct Flatten<O>
where
    O : Iterator,
    O::Item : IntoIterator,
{
    outer : O,
    front_iter : Option<<O::Item as IntoIterator>::IntoIter>,
    back_iter : Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten <O> 
where
    O : Iterator,
    O::Item : IntoIterator,

{
    fn new(iter: O) -> Self {
        Flatten {
            outer : iter,
            inner : None,
        }
    }

}

impl<O> Iterator for Flatten <O> 
where
    O: Iterator,
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        loop{
            if let Some(ref mut front_iter) = self.front_iter {
                if let Some(i) = front_iter.next(){
                    return Some(i);
                }
                self.inner = None;
            }

            if let  next_front_iter = self.outer.next() {
                
            }
            
        }
    }

}

impl<O> DoubleEndedIterator for Flatten <O>
where
    O: Iterator + DoubleEndedIterator,
    O::Item: IntoIterator,
    <O::Item as IntoIterator>::IntoIter : IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        loop{
            if let Some(ref mut inner_iter) = self.inner {
                if let Some(i) = inner_iter.next(){
                    return Some(i);
                }
                self.inner = None;
            }

            let next_inner_iter = self.outer.next()?.into_iter();
            self.inner = Some(next_inner_iter);

        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count(), 0)
    }
    #[test]
    fn once() {
        assert_eq!(flatten(std::iter::once(vec!["a"])).count(), 1)
    }
    #[test]
    fn two() {
        assert_eq!(flatten(std::iter::once(vec!["a", "b"])).count(), 2)
    }
    #[test]
    fn two_way() {
        assert_eq!(flatten(vec![vec!["a"], vec!["b"]]).count(), 2)
    }
}

