use std::sync::atomic::{AtomicBool, Ordering};
use std::cell::UnsafeCell;

const LOCKED:bool = true;
const UNLOCKED:bool = false;

pub struct Mutex<T> {
    value: UnsafeCell<T>,
    locked: AtomicBool
}
unsafe impl<T> Sync for Mutex<T> where T: Send {}

impl<T> Mutex <T>{
    fn new(t: T) -> Self {

        Self{
            value : UnsafeCell::new(t),
            locked : AtomicBool::new(UNLOCKED),
        }
    }

    fn with_lock<R> (&self, f: impl FnOnce(&mut T) -> R) -> R {
        while self.locked.compare_exchange(UNLOCKED,LOCKED, Ordering::Relaxed, Ordering::Relaxed).is_err() 
        {
            self.locked.load(Ordering::Relaxed);
            std::thread::yield_now();
        }

        std::thread::yield_now();
        //SAFETY: we hold the lock so we can create a mutable reference
        let ret = f(unsafe{&mut *self.value.get()});
        self.locked.store(UNLOCKED, Ordering::Relaxed);
        ret
    }

}

use std::thread::spawn;

fn main() {
    let m: &'static _ = Box::leak(Box::new(Mutex::new(0)));

    let handles: Vec<_> = (0..100)
        .map(move |_| {
            spawn(||{
                for _ in 0..1000 {
                    m.with_lock(|v| 
                        *v += 1
                        )
                }

            })
        }).collect();

    for handle in handles {
        handle.join().unwrap();
    }
    assert_eq!(m.with_lock(|v| *v), 100 * 1000);

}


