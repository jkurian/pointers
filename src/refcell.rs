use crate::cell::Cell;
use std::cell::UnsafeCell;

pub struct RefCell<T> {
    value: UnsafeCell<T>,
    state: Cell<RefState>,
}

#[derive(Copy, Clone)]
enum RefState {
    Unshared,
    Exclusive,
    Shared(usize),
}

impl<T> RefCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            state: Cell::new(RefState::Unshared),
        }
    }

    pub fn borrow(&self) -> Option<Ref<'_, T>> {
        match self.state.get() {
            RefState::Unshared => {
                self.state.set(RefState::Shared(1));
                Some(Ref { refcell: self })
            }
            RefState::Shared(n) => {
                self.state.set(RefState::Shared(n + 1));
                Some(Ref { refcell: self })
            }
            RefState::Exclusive => None,
        }
    }

    pub fn borrow_mut(&self) -> Option<RefMut<'_, T>> {
        if let RefState::Unshared = self.state.get() {
            self.state.set(RefState::Exclusive);
            Some(RefMut { refcell: self })
        } else {
            None
        }
    }
}

pub struct Ref<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> std::ops::Deref for Ref<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        /// SAFETY
        unsafe {
            &*self.refcell.value.get()
        }
    }
}

impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Shared(1) => self.refcell.state.set(RefState::Unshared),
            RefState::Shared(n) => self.refcell.state.set(RefState::Shared(n - 1)),
            RefState::Unshared | RefState::Exclusive => unreachable!(),
        }
    }
}

pub struct RefMut<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> std::ops::Deref for RefMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        /// SAFETY
        unsafe {
            &*self.refcell.value.get()
        }
    }
}

impl<T> std::ops::DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        /// SAFETY
        unsafe {
            &mut *self.refcell.value.get()
        }
    }
}

impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Exclusive => self.refcell.state.set(RefState::Unshared),
            RefState::Unshared | RefState::Shared(_) => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {

    use super::RefCell;
    #[test]
    pub fn state() {
        let a = RefCell::new(5);
        let mut_borrow = *a.borrow_mut().unwrap();
        let b = RefCell::new(5);
        let borrow = *a.borrow().unwrap();

        let x = RefCell::new(vec![1, 2, 3, 4]);
        {
            println!("{:?}", *x.borrow().unwrap())
        }

        {
            let a = x.borrow().unwrap();
            let mut my_ref = x.borrow_mut().unwrap();
            my_ref.push(1);
        }
    }
}
