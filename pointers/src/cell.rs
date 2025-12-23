use std::cell::UnsafeCell;
pub struct Cell<T> {
    value: UnsafeCell<T>,
}

impl<T> Cell <T> {
 pub fn new(&self, value: T) -> Self {
     Cell{
         value : UnsafeCell::new(value)
     }

 }

 pub fn set(&self, value: T){
     ///safety: we know no one is concurrently mutating self.value,
     ///(beacause !sync), and it is executing this function instead
     ///safety: we know we're not invalidating any reference because we never give any out
     unsafe{*self.value.get() = value};

 }

 pub fn get(&self) -> T 
 where
 T:Copy,
 {   
     ///safety: we know no one is modifying this value, ince only this thread can mutate
     ///(beacause !sync), and it is executing this function instead
     unsafe{*self.value.get()}

 }
}

#[cfg(test)]
mod test {
    use super::Cell;
    
    #[test]
    fn bad() {
        use std::sync::Arc;
        let x = Arc::new(Cell::new(42));
        let x1 = Arc::clone(&x);
        thread::spawn(|| {
            x1.set(43)
        });
        let x2 = Arc::clone(&x);
        thread::spawn(|| {
            x2.set(45);
        });
    }

}
