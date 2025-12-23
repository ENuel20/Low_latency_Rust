use std::cell::UnsafeCell;
use cre::cell::Cell;

#[derive(Copy, Clone)]
pub enum RefState {
    unshared,
    shared(usize),
    exclusive,
}
pub struct RefCell<T> {
    value: UnsafeCell<T>,
    reference: Cell<RefState>,
}

impl<T> RefCell <T> {
    fn new(&self, value: T) -> Self {
        RefCell{
            value: UnsafeCell::new(value),
            reference: Cell::new(RefState::unshared),
        }
    }

    fn borrow(&self) -> Option<Ref<'_, T>> {
        match self.reference.get() {
            RefState::unshared => {
                self.reference.set(RefState::shared(1));
                Some( Ref{ refcell:self })
            }
            RefState::shared(n) => {
                self.reference.set(RefState::shared(n+1));
                Some( Ref{ refcell:self })
            }
            RefState::exclusive => None
        }
    }

    fn borrow_mut(&self) -> Option<RefMut<'_, T>> {
        if let RefState::unshared = self.reference.get(){
            self.reference.set(RefState::exclusive);
            Some(RefMut{ refcell:self })
        }else{
            None
        }
    }
}

pub struct Ref<'refcell, T>{
    refcell : &'refcell RefCell<T>,
}

impl<T> std::ops::Deref for Ref<'_, T>{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        ///SAFETY: see safety for DerefMut
        unsafe{&*self.refcell.value.get()}
    }
}

impl<T> Drop for Ref<'_, T> 
{
    fn drop(&mut self){
        match self.refcell.reference.get() {
            RefState::exclusive | RefState::unshared => unreachable!(),
            RefState::shared(1) => {
                self.refcell.reference.set(RefState::unshared);
            }
            RefState::shared(n) => {
                self.refcell.reference.set(RefState::shared(n - 1));
            }
        }
    }}

pub struct RefMut<'refcell, T>{
    refcell : &'refcell RefCell<T>,
}

impl<T> std::ops::Deref for RefMut<'_, T>{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        ///SAFETY: see safety for DerefMut
        unsafe{&*self.refcell.value.get()}
    }
}


impl<T> std::ops::DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        ///SAFETY: a deref_mut is only created if no other references have been given out
        ///once it is given out, state is set to exclusive, so no future references are given out
        ///so we have an exclusive lease on the inner value, so mutably derefencing is fine
        unsafe{&mut *self.refcell.value.get()}
    }
}

impl <T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        match self.refcell.reference.get() {
        RefState::shared(_) | RefState::unshared => unreachable!(),
        RefState::exclusive => {
            self.refcell.reference.set(RefState::unshared);
        }
    }
}



}
