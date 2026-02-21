use std::ptr::NonNull;
use std::thread;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::Ordering::Release;
use std::sync::atomic::Ordering::Acquire;
use std::sync::atomic::fence;
use std::ops::Deref;

struct ArcData<T> {
    ref_count : AtomicUsize,
    data : T,
}

pub struct Arc<T> {
    ptr : NonNull<ArcData<T>>,
}

unsafe impl<T: Send + Sync> Send for Arc<T> {}
unsafe impl<T: Send + Sync> Sync for Arc<T> {}

impl<T> Arc <T> {
    pub fn new(data: T) -> Arc<T> {
        Arc { ptr :  NonNull::from(Box::leak(Box::new(ArcData{
            ref_count : AtomicUsize::new(1),
            data,
        })))
        }
    }

    fn data (&self) -> &ArcData<T> {
        unsafe{self.ptr.as_ref()}
    }

    pub fn get_mut(arc : &mut Self) -> Option<&mut T> {
        if arc.data().ref_count.load(Relaxed) == 1 {
            fence(Acquire);
            // Safety: Nothing else can access the data, since
            // there's only one Arc, to which we have exclusive access.
            unsafe{ Some(&mut arc.ptr.as_mut().data) }
        }else {
            None
        }
    }
}

impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        //TODO: handle overflow
        if self.data().ref_count.fetch_add(1, Relaxed) >= usize::MAX / 2 {
            std::process::abort();
        }
        Arc {
            ptr : self.ptr,
        }
    }
}

impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        if self.data().ref_count.fetch_sub(1, Release) == 1{
            fence(Acquire);
            unsafe {
                drop(Box::from_raw(self.ptr.as_ptr()));
            }
        }
    }
    
} 

impl<T> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.data().data
    }
}

fn main() {
    #[test]
    fn test() {
        static NUM_DROPS : AtomicUsize = AtomicUsize::new(0);

        struct DetectDrop;

        impl Drop for DetectDrop {
            fn drop(&mut self) {
                NUM_DROPS.fetch_add(1, Relaxed);
            }
        }
        
        //create an Arc sharing a string an object and containing 
        // a string and a DetectDrop, to detect wen it is dropped
        let x = Arc::new(("hello", DetectDrop));
        let y = x.clone();

        //send to another and use it there
        
        let t = thread::spawn( move|| {
            assert_eq!(x.0, "hello");
        });

        //in parallel y sould still be usable here
        assert_eq!(y.0, "hello");

        //wait for the thread to finish.
        t.join().unwrap();

        //One Arc, x, sould e dropped by now
        //we still have y, so te object shouldn't have been dropped yet
        assert_eq!(NUM_DROPS.load(Relaxed), 0);

        //drop the remaining arc
        drop(y);

        assert_eq!(NUM_DROPS.load(Relaxed), 1);
    } 
}
