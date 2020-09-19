use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        unsafe { *self.value.get() = value }
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { *self.value.get() }
    }
}
#[cfg(test)]
mod test {
    use super::Cell;

    #[test]
    fn bad() {
        // use std::sync::Arc;
        // let myCell = Arc::new(Cell::new(5));
        // let a = Arc::clone(&myCell)
        // std::thread::spawn(|| a.set(10));
        // let b = Arc::clone(&myCell)
        // std::thread::spawn(|| b.set(20));
    }
}
