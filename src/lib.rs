/// Macro for creating a `Box<T>` from a value
///
/// # Example
/// ```
/// use hurry::boxx;
/// let x = boxx!(5);
/// assert_eq!(*x, 5);
/// ```
#[macro_export]
macro_rules! boxx {
    ($val:expr) => {
        Box::new($val)
    };
}

/// Macro for creating an `Rc<T>` from a value
///
/// # Example
/// ```
/// use hurry::rc;
/// let x = rc!(5);
/// assert_eq!(*x, 5);
/// ```
#[macro_export]
macro_rules! rc {
    ($val:expr) => {
        std::rc::Rc::new($val)
    };
}

/// Macro for creating an `Arc<T>` from a value
///
/// # Example
/// ```
/// use hurry::arc;
/// let x = arc!(5);
/// assert_eq!(*x, 5);
/// ```
#[macro_export]
macro_rules! arc {
    ($val:expr) => {
        std::sync::Arc::new($val)
    };
}

/// Macro for creating a `Rc<RefCell<T>>` from a value
///
/// # Example
/// ```
/// use hurry::rc_refcell;
/// let x = rc_refcell!(5);
/// *x.borrow_mut() = 10;
/// assert_eq!(*x.borrow(), 10);
/// ```
#[macro_export]
macro_rules! rc_refcell {
    ($val:expr) => {
        std::rc::Rc::new(std::cell::RefCell::new($val))
    };
}

/// Macro for creating an `Arc<Mutex<T>>` from a value
///
/// # Example
/// ```
/// use hurry::arc_mutex;
/// let x = arc_mutex!(5);
/// *x.lock().unwrap() = 10;
/// assert_eq!(*x.lock().unwrap(), 10);
/// ```
#[macro_export]
macro_rules! arc_mutex {
    ($val:expr) => {
        std::sync::Arc::new(std::sync::Mutex::new($val))
    };
}

/// Macro for creating an `Arc<RwLock<T>>` from a value
///
/// # Example
/// ```
/// use hurry::arc_rwlock;
/// let x = arc_rwlock!(5);
/// *x.write().unwrap() = 10;
/// assert_eq!(*x.read().unwrap(), 10);
/// ```
#[macro_export]
macro_rules! arc_rwlock {
    ($val:expr) => {
        std::sync::Arc::new(std::sync::RwLock::new($val))
    };
}

/// Macro for creating a `Mutex<T>` from a value
///
/// # Example
/// ```
/// use hurry::mutex;
/// let x = mutex!(5);
/// *x.lock().unwrap() = 10;
/// assert_eq!(*x.lock().unwrap(), 10);
/// ```
#[macro_export]
macro_rules! mutex {
    ($val:expr) => {
        std::sync::Mutex::new($val)
    };
}

/// Macro for creating a `RwLock<T>` from a value
///
/// # Example
/// ```
/// use hurry::rwlock;
/// let x = rwlock!(5);
/// *x.write().unwrap() = 10;
/// assert_eq!(*x.read().unwrap(), 10);
/// ```
#[macro_export]
macro_rules! rwlock {
    ($val:expr) => {
        std::sync::RwLock::new($val)
    };
}

/// Macro for creating a `Cell<T>` from a value
///
/// # Example
/// ```
/// use hurry::cell;
/// let x = cell!(5);
/// x.set(10);
/// assert_eq!(x.get(), 10);
/// ```
#[macro_export]
macro_rules! cell {
    ($val:expr) => {
        std::cell::Cell::new($val)
    };
}

/// Macro for creating a `RefCell<T>` from a value
///
/// # Example
/// ```
/// use hurry::refcell;
/// let x = refcell!(5);
/// *x.borrow_mut() = 10;
/// assert_eq!(*x.borrow(), 10);
/// ```
#[macro_export]
macro_rules! refcell {
    ($val:expr) => {
        std::cell::RefCell::new($val)
    };
}

/// Macro for creating a `Pin<Box<T>>` from a value
///
/// # Example
/// ```
/// use hurry::pin_box;
/// let x = pin_box!(5);
/// assert_eq!(*x, 5);
/// ```
#[macro_export]
macro_rules! pin_box {
    ($val:expr) => {
        Box::pin($val)
    };
}

/// Macro for creating a `Vec<Box<T>>` from multiple values
///
/// # Example
/// ```
/// use hurry::vec_box;
/// let v = vec_box![1, 2, 3];
/// assert_eq!(*v[0], 1);
/// assert_eq!(*v[1], 2);
/// assert_eq!(*v[2], 3);
/// ```
#[macro_export]
macro_rules! vec_box {
    ($($val:expr),* $(,)?) => {
        vec![$(Box::new($val)),*]
    };
}

/// Macro for creating a `Vec<Rc<T>>` from multiple values
///
/// # Example
/// ```
/// use hurry::vec_rc;
/// let v = vec_rc![1, 2, 3];
/// assert_eq!(*v[0], 1);
/// assert_eq!(*v[1], 2);
/// assert_eq!(*v[2], 3);
/// ```
#[macro_export]
macro_rules! vec_rc {
    ($($val:expr),* $(,)?) => {
        vec![$(std::rc::Rc::new($val)),*]
    };
}

/// Macro for creating a `Vec<Arc<T>>` from multiple values
///
/// # Example
/// ```
/// use hurry::vec_arc;
/// let v = vec_arc![1, 2, 3];
/// assert_eq!(*v[0], 1);
/// assert_eq!(*v[1], 2);
/// assert_eq!(*v[2], 3);
/// ```
#[macro_export]
macro_rules! vec_arc {
    ($($val:expr),* $(,)?) => {
        vec![$(std::sync::Arc::new($val)),*]
    };
}

/// Macro for creating an owned `Cow<T>` from a value
///
/// # Example
/// ```
/// use hurry::cow_owned;
/// let x: std::borrow::Cow<'_, str> = cow_owned!("hello".to_string());
/// assert_eq!(&*x, "hello");
/// ```
#[macro_export]
macro_rules! cow_owned {
    ($val:expr) => {
        std::borrow::Cow::Owned($val)
    };
}

/// Macro for creating a borrowed `Cow<T>` from a reference
///
/// # Example
/// ```
/// use hurry::cow_borrowed;
/// let x = cow_borrowed!("hello");
/// assert_eq!(&*x, "hello");
/// ```
#[macro_export]
macro_rules! cow_borrowed {
    ($val:expr) => {
        std::borrow::Cow::Borrowed($val)
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_boxx_macro() {
        let x = boxx!(42);
        assert_eq!(*x, 42);
    }

    #[test]
    fn test_rc_macro() {
        let x = rc!(42);
        let y = x.clone();
        assert_eq!(*x, 42);
        assert_eq!(*y, 42);
    }

    #[test]
    fn test_arc_macro() {
        let x = arc!(42);
        let y = x.clone();
        assert_eq!(*x, 42);
        assert_eq!(*y, 42);
    }

    #[test]
    fn test_rc_refcell_macro() {
        let x = rc_refcell!(42);
        *x.borrow_mut() = 100;
        assert_eq!(*x.borrow(), 100);
    }

    #[test]
    fn test_arc_mutex_macro() {
        let x = arc_mutex!(42);
        *x.lock().unwrap() = 100;
        assert_eq!(*x.lock().unwrap(), 100);
    }

    #[test]
    fn test_arc_rwlock_macro() {
        let x = arc_rwlock!(42);
        *x.write().unwrap() = 100;
        assert_eq!(*x.read().unwrap(), 100);
    }

    #[test]
    fn test_mutex_macro() {
        let x = mutex!(42);
        *x.lock().unwrap() = 100;
        assert_eq!(*x.lock().unwrap(), 100);
    }

    #[test]
    fn test_rwlock_macro() {
        let x = rwlock!(42);
        *x.write().unwrap() = 100;
        assert_eq!(*x.read().unwrap(), 100);
    }

    #[test]
    fn test_cell_macro() {
        let x = cell!(42);
        x.set(100);
        assert_eq!(x.get(), 100);
    }

    #[test]
    fn test_refcell_macro() {
        let x = refcell!(42);
        *x.borrow_mut() = 100;
        assert_eq!(*x.borrow(), 100);
    }

    #[test]
    fn test_pin_box_macro() {
        let x = pin_box!(42);
        assert_eq!(*x, 42);
    }

    #[test]
    fn test_vec_box_macro() {
        let v = vec_box![1, 2, 3];
        assert_eq!(*v[0], 1);
        assert_eq!(*v[1], 2);
        assert_eq!(*v[2], 3);
    }

    #[test]
    fn test_vec_rc_macro() {
        let v = vec_rc![1, 2, 3];
        assert_eq!(*v[0], 1);
        assert_eq!(*v[1], 2);
        assert_eq!(*v[2], 3);
    }

    #[test]
    fn test_vec_arc_macro() {
        let v = vec_arc![1, 2, 3];
        assert_eq!(*v[0], 1);
        assert_eq!(*v[1], 2);
        assert_eq!(*v[2], 3);
    }

    #[test]
    fn test_cow_owned_macro() {
        let x: std::borrow::Cow<'_, str> = cow_owned!("hello".to_string());
        assert_eq!(&*x, "hello");
    }

    #[test]
    fn test_cow_borrowed_macro() {
        let x = cow_borrowed!("hello");
        assert_eq!(&*x, "hello");
    }
}
