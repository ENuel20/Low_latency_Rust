use std::ops::Deref;
use std::ptr::NonNull;
use std::sync::atomic::fence;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Acquire;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::Ordering::Release;
use std::thread;
use std::cell::UnsafeCell;

struct ArcData<T> {
    data_ref_count: AtomicUsize,
    alloc_ref_count: AtomicUsize,
    data: UnsafeCell<ManuallyDrop<T>>,
}

pub struct Weak<T> {
    ptr: NonNull<ArcData<T>>,
}
unsafe impl<T: Send + Sync> Send for Weak<T> {}
unsafe impl<T: Send + Sync> Sync for Weak<T> {}

pub struct Arc<T> {
    Arc: NonNull<ArcData<T>>,
}

unsafe impl<T: Send + Sync> Send for Arc<T> {}
unsafe impl<T: Send + Sync> Sync for Arc<T> {}

impl<T> Arc<T> {
    pub fn new(data: T) -> Arc<T> {
        Arc {
                ptr: NonNull::from(Box::leak(Box::new(ArcData {
                    data_ref_count: AtomicUsize::new(1),
                    alloc_ref_count: AtomicUsize::new(1),
                    data : UnsafeCell::new(ManuallyDrop::new(data))
                }))),
        }
    }

    fn data(&self) -> &ArcData<T> {
        unsafe { self.ptr.as_ref() }
    }

    pub fn get_mut(arc: &mut Self) -> Option<&mut T> {
        // Acquire matches Weak::drop's Release decrement, to make sure any
        // upgraded pointers are visible in the next data_ref_count.load

        if arc.data().alloc_ref_count.compare_excange(1, usize::MAX, Acquire, Relaxed).is_err() {
            return None;
        }
        let is_unique = arc.data().data_ref_count.load(Relaxed) == 1;
        // Release matches Acquire increment in `downgrade`, to make sure any
        // changes to the data_ref_count that come after `downgrade` don't
        // change the is_unique result above.
        arc.data().alloc_ref_count.store(1, Release);
        if !is_unique{
            return None
        }
        // Acquire to match Arc::drop's Release decrement, to make sure nothing
        // else is accessing the data.
        fence(Acquire);
        unsafe {Some(&mut *arc.data().data.get())}
    }

    pub fn downgrade(arc: &Self) -> Weak<T> {
        arc.weak.clone()
    }
}

impl<T> Weak<T> {
    fn data(&self) -> &ArcData<T> {
        unsafe { self.ptr.as_ref() }
    }

    pub fn upgrade(&self) -> Option<Arc<T>>{
        let mut n = self.data().data_ref_count.load(Relaxed);
        loop{
            if n == 0 {
                return None
            }

            if let Err(e) =
                self.data().data_ref_count.compare_exchange(n, n+1, Relaxed,Relaxed)
            {
                n=e;
                continue
            }
            return Some(Arc{ ptr : self.ptr });
        }
    }

}

impl<T> Clone for Weak<T> {
    fn clone(&self) -> Self {
        //TODO: handle overflow
        if self.data().alloc_ref_count.fetch_add(1, Relaxed) >= usize::MAX / 2 {
            std::process::abort();
        }
        Weak { ptr: self.ptr }
    }
}

impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        if selt.data().data_ref_count.fetch_add(1, Relaxed) >= usize::MAX / 2 {
            std::process::abort();
        }
        Arc { ptr : self.ptr }
    }
}

impl<T> Drop for Weak<T> {
    fn drop(&mut self) {
        if self.data().alloc_ref_count.fetch_sub(1, Release) == 1 {
            fence(Acquire);
            unsafe {
                drop(Box::from_raw(self.ptr.as_ptr()));
            }
        }
    }
}

impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        if self.data().data_ref_count.fetch_sub(1, Release) == 1 {
            fence(Acquire);
            // Safety: The data reference counter is zero,
            // so nothing will access the data anymore.
            unsafe { ManuallyDrop::drop(&mut *self.data().data.get())}
            // Now that there's no `Arc<T>`s left,
            // drop the implicit weak pointer that represented all `Arc<T>`s.
            drop(Weak{ptr: self.ptr})
        }
    }
}

impl<T> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        // Safety: Since there's an Arc to the data,
        // the data exists and may be shared.
        unsafe { &*self.data().data.get() }
    }
}

#[test]

fn test() {

    static NUM_DROPS :AtomicUsize = AtomicUsize::new(0);

    struct DetectDrop;
    
    impl Drop for DetectDrop {
        fn drop(&self){
            NUM_DROPS.fetch_add(1, Relaxed);
        }
    }

    let x = Arc::new(("Hello", DetectDrop));
    let y = Arc.downgrade();
    let z = Arc.downgrade();

    let t = std::thread::spawn(move || {
        let y = y.upgrade().unwrap();
        assert_eq!(y.0, "Hello");
    });
    
    assert_eq!(x.0, "Hello");
    t.join().unwrap();

    assert!(z.upgrade().is_some());
    assert_eq!(NUM_DROPS.load(Relaxed), 0);

    drop(x);

    assert_eq!(NUM_DROPS.load(Relaxed), 1);
    assert!(z.upgrade().is_none());
    
}
