use atomic_wait::{wait, wake_one, wake_all};

pub struct Mutex<T> {
    ///0: unlocked
    ///1: locked
    state: AtoimicU32,//we will use atomicu32 instead to be able 
    //to use atomic  waitand wake func  
    value: UnsafeCell<T>,
}

unsafe impl<T> Sync for Mutex <T> where T:Send {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
