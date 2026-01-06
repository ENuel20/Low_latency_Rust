use std::ptr::NonNull;
use std::alloc;

pub struct MyVec <T> {
    ptr: NonNull<T>,
    capacity: usize,
    len: usize
}

impl<T> MyVec <T> {
    pub fn new() -> Self {
        MyVec{
            ptr: NonNull::dangling(),
            capacity: 0,
            len: 0,
        }
    }
    
    pub fn push(&mut self, item:T) {
        assert_ne!(std::mem::size_of::<T>(), 0);
        if self.capacity == 0 {
            let layout = alloc::Layout::array::<T>(4).expect("could not accolate");
            //SAFETY:the layout is hardcoded to be 4 * size_of<T>
            //and size_of<T> is > 0
            let ptr = unsafe {alloc::alloc(layout)} as *mut T;
            let ptr = NonNull::new(ptr).expect("could not allocate");
            unsafe {ptr.as_ptr().write(item)}
            self.ptr = ptr;
            self.capacity = 4;
            self.len = 1;
        }
        else if self.len < self.capacity {
            let offset = self.len
                .checked_mul(std::mem::size_of::<T>())
                .expect("cannot reach memory location");
            assert!(offset < isize::MAX as usize, "Wrapped isize");
            //SAFETY: offset cannot wrapped around and pointer is pointing to valid memory
            //and writing to an ofset at self.len is valid
            unsafe {self.ptr.as_ptr().add(self.len).write(item)}
            self.len += 1;
        }
        else {
            debug_assert!(self.len == self.capacity);
            let new_capacity = self.capacity.checked_mul(2).expect("'capacity wrapped");
            let align = std::mem::align_of::<T>();
            let size = std::mem::size_of::<T>() * self.capacity;
            size.checked_add(size % align).expect("cant allocate");
            unsafe {
                let layout = alloc::Layout::from_size_align_unchecked(size, align);
                let new_size = std::mem::size_of::<T>() * new_capacity;
                let ptr = alloc::realloc(self.ptr.as_ptr() as *mut u8, layout, new_size);
                let ptr = NonNull::new(ptr as *mut T).expect("could not reallocate");
                ptr.as_ptr().add(self.len).write(item);
                self.ptr = ptr;
                self.len += 1;
                self.capacity = new_capacity;
            }

        }
    }

    pub fn get(&self, index:usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }
        Some(unsafe {&*self.ptr.as_ptr().add(index)})

    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T> Drop for MyVec <T> {
    fn drop(&mut self) {
        unsafe {
            std::ptr::drop_in_place(std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len));
            let layout = std::alloc::Layout::from_size_align_unchecked
                (std::mem::size_of::<T>() * self.capacity, std::mem::align_of::<T>());
            std::alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout)

        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut vec = MyVec::<usize>::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        vec.push(4);
        vec.push(5);

        for n in 0..vec.len() {
            assert_eq!(vec.get(n), Some(&(n + 1)));
        }    
        assert_eq!(vec.capacity(), 8);
        assert_eq!(vec.len(), 5);
    }
}
