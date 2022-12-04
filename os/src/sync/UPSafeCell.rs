use core::cell;

/// On the basis of RefCell, a UPSafeCell is encapsulated.
/// The meaning of its name is: it allows us to safely use variable global variables on a single core.
/// It provides the same internal mutability and runtime borrow checking as RefCell,
/// but is more strict: calling `exclusive_access` can get exclusive access to the data it wraps.
pub struct UPSafeCell<T> {
    // inner data
    inner: cell::RefCell<T>,
}

/// Currently our kernel only runs on a single core, so there is no need to care about any data race/synchronization issues caused by multi-core.
/// Based on RefCell, a runtime borrow checking function is provided, which satisfies Rust's basic constraints on borrowing and ensures memory safety.
unsafe impl<T> Sync for UPSafeCell<T> {}

impl<T> UPSafeCell<T> {
    /// User is responsible to guarantee that inner struct is only used in uniprocessor.
    /// It is hoped that when users create a UPSafeCell, they will never violate the following pattern when accessing the data wrapped in UPSafeCell:
    ///     `exclusive_access` is called before access, and the borrowed token is destroyed after access for the next access.
    /// When the user violates the above mode, such as forgetting to destroy after accessing and opening the next access, the program will panic and exit.
    pub unsafe fn new(value: T) -> Self {
        Self {
            inner: cell::RefCell::new(value),
        }
    }

    /// Panic if the data has been borrowed.
    pub fn exclusive_access(&self) -> cell::RefMut<'_, T> {
        self.inner.borrow_mut()
    }
}
