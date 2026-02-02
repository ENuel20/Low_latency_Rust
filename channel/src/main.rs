use std::mem::MaybeUninit;
use std::thread;
use std::cell::UnsafeCell;
use std::thread::Thread;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Release, Acquire, Relaxed};
use std::sync::atomic::AtomicU8;
use std::marker::PhantomData;
pub struct Channel<T>{
    message : UnsafeCell<MaybeUninit<T>>,
    ready : AtomicBool,
}

pub struct Sender<'a, T> {
    channel : &'a Channel<T>,
    recieving_thread : Thread,
}

pub struct Reciever<'a, T> {
    channel : &'a Channel<T>,
    _no_send : PhantomData<*const()>,
}

unsafe impl<T> Sync for Channel<T> where T:Send {}

impl<T> Channel <T> {
    pub fn new() -> Self {
        Self{
            message : UnsafeCell::new(MaybeUninit::uninit()),
            ready : AtomicBool::new(false),
        }
    }

    pub fn split<'a>(&'a mut self) -> (Sender<'a, T>, Reciever<'a, T>) {
        *self = Self::new();
        (Sender { 
            channel: self,
            recieving_thread : thread::current()
        }, 
        Reciever { 
            channel: self,
            _no_send : PhantomData
        })
    }
}

impl<T> Sender <'_, T> {
    pub fn send(self, message :T) {
        unsafe {(*self.channel.message.get()).write(message)};
        self.channel.ready.store(true,Release);
        self.recieving_thread.unpark();
    }
}

impl<T> Reciever <'_, T> {
    pub fn is_ready(&self) -> bool {
        self.channel.ready.load(Relaxed) 
    }
    pub fn recieve(self) -> T {
        if !self.channel.ready.swap(false, Acquire){
            thread::park();
        }
        unsafe {(*self.channel.message.get()).assume_init_read()}
    }
}

impl<T> Drop for Channel <T> {
    fn drop(&mut self) {
        if *self.ready.get_mut() {
            unsafe{self.message.get_mut().assume_init_drop()}
        }
    }
}

fn main() {
    let mut channel = Channel::new();
    thread::scope(|s| {
        let (sender, reciever) = channel.split();
        let t = thread::current();
        s.spawn(move || {
            sender.send("helo");
            t.unpark();
        });
        assert_eq!(reciever.recieve(), "helo");
    });
}


