use std::sync::Condvar;
use std::sync::Mutex;
use std::collections::VecDeque;
pub struct Channel<T> {
    queue : Mutex<VecDeque<T>>,
    item_ready : Condvar,
}

impl<T> Channel <T> {
    pub fn new() -> Self {
        Channel{
            queue : Mutex::new(VecDeque::new()),
            item_ready : Condvar::new()
        }
    }

    pub fn send(&self, messages: T) {
        self.queue.lock().unwrap().push_back(messages);
        self.item_ready.notify_one();
    }
    
    pub fn recieve(&self) -> T {
        let mut b = self.queue.lock().unwrap();
        loop {
            if let Some(messages) = b.pop_front() {
                return messages
            }
            b = self.item_ready.wait(b).unwrap();
        }
    }
}
