struct Channel<T>{
    message : UnsafeCell<MaybeUninit<T>>,
    ready : AtomicBool,
}

pub struct Sender<T> {
    channel : Arc<Channel<T>>,
}

pub struct Reciever<T> {
    channel : Arc<Channel<T>>,
}

unsafe impl<T> Sync for Channel<T> where T:send {}

pub fn channel<T>() -> (Sender<T>, Reciever<T>) {
    let a = Arc::new(Channel{
        message : UnsafeCell::new(MaybeUninit::uninit()),
        ready : AtomicBool::new(false),
    });
    (Sender{channel : a.clone()}, Reciever{channel : a})
}

impl<T> Sender <T> {
    pub fn send(self, message :T) {
        (*self.channel.message.get()).write(message);
        self.channel.ready.store(true,Release);
    }
}

impl<T> Reciever <T> {
    pub fn is_ready(&self) -> bool {
        self.channel.ready.load(Relaxed) 
    }
    pub fn recieve(self) -> T {
        if !self.channel.ready.swap(false, Acquire){
            panic!("No message Available");
        }
        unsafe {(*self.channel.message.get()).assume_init_read()}
    }
}

impl<T> Drop for Channel <T> {
    fn drop(&mut self) {
        if *self.ready.get_mut() {
            unsafe{(*self.message.get().assume_init_drop())}
        }
    }
}

fn main() {
    thread::scope(|s| {
        let (sender, reciever) = Channel();
        let t = thread::current();
        s.spawn(move || {
            sender.send("helo");
            t = unpark();
        });

        while !reciever.is_ready() {
            thread::park();
        }
    });

    assert_eq!(reciever.recieve(), "helo");
}


