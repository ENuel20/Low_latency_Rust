use std::sync::atomic::compiler_fence;
use std::sync::atomic::{AtomicU64,AtomicBool};
use std::sync::atomic::Ordering::{Relaxed,Acquire,Release};
use std::thread;


fn main() {
    let locked = AtomicBool::new(false);
    let counter = AtomicU64::new(0);

    thread::scope(|s|{
        for _ in 0..4{
        s.spawn(||
            for x in  0..1000000 {
                locked.store(true, Relaxed);
                compiler_fence(Acquire);

                let old = counter.load(Relaxed);
                let new = old + 1;
                counter.store(new, Relaxed);

                compiler_fence(Release);
                locked.store(false, Relaxed);

            
        });
        }
    });
    println!("time elapsed {:?}", counter.into_inner());
}
