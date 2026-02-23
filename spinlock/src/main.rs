use std::sync::atomic::{AtomicBool, Ordering::{Release, Acquire}};
use std::cell::UnsafeCell;
use std::ops::DerefMut;
use std::ops::Deref;
use std::thread;

struct Guard<'a, T>{
    lock :&'a SpinLock< T>
}
impl<T> Deref for Guard<'_, T>{
    type Target = T;
    fn deref(&self) -> &T {
        unsafe{&*self.lock.value.get()}
    }
}

impl<T> DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe{&mut *self.lock.value.get()}
    }
}

impl<T> Drop for Guard<'_, T>{
    fn drop(&mut self) {
        self.lock.locked.store(false, Release);
    }
}

pub struct SpinLock<T> {
    locked : AtomicBool,
    value : UnsafeCell<T>,
}
unsafe impl<T> Sync for SpinLock<T> where T:Send {}


impl<T> SpinLock<T> {
    pub const fn new(value : T) -> Self {
        Self{
            locked : AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    pub fn lock(&self) -> Guard<'_, T> {
        while self.locked.swap(true, Acquire) {
            std::hint::spin_loop();
        }
        Guard{lock : self}
    }
}

fn main() {
    let m = SpinLock::new(Vec::new());
    thread::scope(|s| {
        s.spawn(|| {
            m.lock().push(1);
        });
        s.spawn(|| {
            m.lock().push(2);
            m.lock().push(3);
        });
        
    });
    let g = m.lock();
    assert!(g.as_slice() == [1,2,3] || g.as_slice() == [3,2,1]);


}
