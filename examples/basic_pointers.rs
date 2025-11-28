// Example demonstrating basic pointer types
use hurry::*;

fn main() {
    // Box - heap allocation for a single value
    let boxed = boxx!(42);
    println!("Boxed value: {}", boxed);

    // Rc - reference counted pointer for shared ownership
    let shared = rc!(String::from("hello"));
    let _clone1 = shared.clone();
    let _clone2 = shared.clone();
    println!(
        "Shared string: {}, references: {}",
        shared,
        std::rc::Rc::strong_count(&shared)
    );

    // Arc - atomic reference counted pointer for thread-safe shared ownership
    let atomic = arc!(vec![1, 2, 3]);
    let _thread_clone = atomic.clone();
    println!(
        "Atomic vec: {:?}, references: {}",
        atomic,
        std::sync::Arc::strong_count(&atomic)
    );

    // Pin<Box<T>> - pinned heap allocation
    let pinned = pin_box!(100);
    println!("Pinned value: {}", pinned);

    // Vectors of boxed values
    let boxes = vec_box![1, 2, 3, 4, 5];
    println!(
        "Vector of boxes: {:?}",
        boxes.iter().map(|b| **b).collect::<Vec<_>>()
    );
}
