use std::mem::MaybeUninit;
use std::cell::UnsafeCell;
use std::thread;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Release, Acquire, Relaxed};

pub struct Channel<T> {
    queue : UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
    in_use: AtomicBool,
}

unsafe impl<T> Sync for Channel <T> where T : Send {}

impl<T> Channel <T> {
    pub const fn new() -> Self {
        Channel {
            queue : UnsafeCell::new(MaybeUninit::uninit()),
            ready : AtomicBool::new(false),
            in_use : AtomicBool::new(false),
        }
    }
    ///only call this once
    pub fn send(&self, messages: T) {
        if self.in_use.swap(true, Relaxed){
            panic!("cant send more than one message");
        }
        unsafe {(*self.queue.get()).write(messages)};
        self.ready.store(true, Release);
    }

    pub fn is_ready(&self) -> bool {
        self.ready.load(Relaxed)
    }
    
    ///panics if no message is available
    ///or message has been consumed
    ///
    ///tip: check is_ready flag
    pub fn recieve(&self) -> T {
        if !self.ready.swap(false, Acquire) {
            panic!("no message available")
        }
        // SAFETY:we have checked and reset the ready flag
        unsafe{(*self.queue.get()).assume_init_read()}
    }
}

impl<T> Drop for Channel <T> {
    fn drop(&mut self) {
        if *self.ready.get_mut(){
            unsafe {self.queue.get_mut().assume_init_drop()}
        }
    }
}
fn main() {
    let channel = Channel::new();
    let t = thread::current();
    thread::scope(|s|{
        s.spawn(|| {
            channel.send("hello, world");
            t.unpark();
        });
        while !channel.is_ready(){
            thread::park();
        }
        assert_eq!(channel.recieve(), "hello, world")
    });
}
