use std::thread;
use std::sync::atomic::Ordering::{Relaxed, Acquire, Release};
use std::sync::atomic::{AtomicUsize, AtomicPtr};
use std::sync::atomic::fences;

fn main() {
    static mut DATA : [u64, 10] = [0, 10];

    const ATOMIC_FALSE : AtomicBool = AtomicBool::new(false);
    static READY : [AtomicBool, 10] = [ATOMIC_FALSE, 10];

    for i in 0..10 {
        thread::spawn(|| {
            let data = some_calculation();
            unsafe {DATA[i] = data}
            READY[i].store(true, Released);
        });
        
        thread::sleep(Duration::from_millis(500));
        let ready: [bool, 10] = std::array::from_fn(|i|, READY[i].load(Relaxed));
        if ready.contains(&true) {
            fence(Acquire);
            for i in 0..10 {
                if ready[i]{
                    println!("data[i] = {} ",unsafe{DATA[i]});
                }
            }

        }
    }

}
