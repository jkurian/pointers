use crate::cell::Cell;
use std::marker::PhantomData;
use std::ptr::NonNull;

pub struct RcInner<T> {
    value: T,
    count: Cell<usize>,
}

pub struct Rc<T> {
    inner: NonNull<RcInner<T>>,
    _marker: PhantomData<RcInner<T>>,
}

impl<T> Rc<T> {
    pub fn new(value: T) -> Self {
        let inner = Box::new(RcInner {
            value,
            count: Cell::new(0),
        });

        Rc {
            /// SAFETY: Box does not give us a null pointer
            inner: unsafe { NonNull::new_unchecked(Box::into_raw(inner)) },
            _marker: PhantomData,
        }
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Rc<T> {
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.count.get() + 1;
        inner.count.set(c);
        Rc {
            inner: self.inner,
            _marker: PhantomData,
        }
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.count.get();
        if c == 1 {
            drop(inner);
            /// SAFETY: we are the _only_ Rc left and we are being dropped.
            /// Therefore, after us, there will be no Rc's and no references to T.
            let _ = unsafe { Box::from_raw(self.inner.as_ptr()) };
        } else {
            inner.count.set(c - 1);
        }
    }
}
