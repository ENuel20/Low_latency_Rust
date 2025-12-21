#![feature(dropck_eyepatch)]

use std::marker::PhantomData;
use std::ptr::NonNull;

pub struct Boks<T> {
    p : NonNull<T>,
    _t: PhantomData<T>,
}

unsafe impl<#[may_dangle] T> Drop for Boks <T> {
    fn drop (&mut self) {
        ///Safety: p was constructed from a Box in the first place, and has not been freed
        ///otherwise since sel still exist (otherwise drop will not be called)
        unsafe{Box::from_raw(self.p.as_mut())};
    }

}
impl<T> Boks <T> {
    fn ny(t:T) -> Self {
        Boks {
            p : unsafe {NonNull::new_unchecked(Box::into_raw(Box::new(t)))},
            _t: Default::default()
        }
    }
}

impl<T> std::ops::Deref for Boks<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        ///Safety: is guarranteed ince it was constructed from a valid T, and turned into a pointer
        ///through Box which creates aligned pointers, and hasn't been freed since self is alive
        unsafe{&*self.p.as_ref()}
    }
    
}

impl<T> std::ops::DerefMut for Boks<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        ///Safety: is guarranteed ince it was constructed from a valid T, and turned into a pointer
        ///through Box which creates aligned pointers, and hasn't been freed since self is alive
        ///also since we have &mut self, no other mutable reference has been given out to p.
        unsafe{&mut *self.p.as_mut()}
    }
}
use std::fmt::Debug;

pub struct Oisann <T:Debug>(T);

impl<T:Debug> Drop for Oisann <T> {
    fn drop (&mut self){
        println!("{:?}",self.0);
    }
}

fn main() {
    let x = 43;
    let p = Boks::ny(x);
    println!("{:?}",*p);

    let mut y = 78;
    let a = Boks::ny(&mut y);
    println!("{:?}",y);

    let z = 46;
    let c = Boks::ny(Oisann(z));
    println!("{:?}", z);

    let s = String::from("hei");
    let mut boks1 = Boks::ny(&*s);
    let boks2: Boks<&'static str> = Boks::ny("heisann");
    boks1 = boks2;
}


