// Example demonstrating interior mutability patterns
use hurry::{arc_mutex, arc_rwlock, rc_refcell};
use std::thread;

fn main() {
    // Single-threaded interior mutability with Rc<RefCell<T>>
    let counter = rc_refcell!(0);

    // Multiple references can modify the same data
    let counter_clone = std::rc::Rc::clone(&counter);
    *counter_clone.borrow_mut() += 1;
    *counter.borrow_mut() += 1;

    println!("Counter value: {}", counter.borrow());

    // Thread-safe interior mutability with Arc<Mutex<T>>
    let shared_counter = arc_mutex!(0);
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = std::sync::Arc::clone(&shared_counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Thread-safe counter: {}", shared_counter.lock().unwrap());

    // Read-write lock for concurrent reads
    let data = arc_rwlock!(vec![1, 2, 3, 4, 5]);
    let data_clone = std::sync::Arc::clone(&data);

    // Multiple readers can access simultaneously
    let reader = thread::spawn(move || {
        let values = data_clone.read().unwrap();
        println!("Reader sees: {:?}", *values);
    });

    reader.join().unwrap();

    // Single writer has exclusive access
    {
        let mut values = data.write().unwrap();
        values.push(6);
    }

    println!("Final data: {:?}", data.read().unwrap());
}
