// Example demonstrating basic pointer types
use hurry::{arc, boxx, pin_box, rc, vec_box};

fn main() {
    // Box - heap allocation for a single value
    let boxed = boxx!(42);
    println!("Boxed value: {boxed}");

    // Rc - reference counted pointer for shared ownership
    let shared = rc!(String::from("hello"));
    let _clone1 = std::rc::Rc::clone(&shared);
    let _clone2 = std::rc::Rc::clone(&shared);
    let ref_count = std::rc::Rc::strong_count(&shared);
    println!("Shared string: {shared}, references: {ref_count}");

    // Arc - atomic reference counted pointer for thread-safe shared ownership
    let atomic = arc!(vec![1, 2, 3]);
    let _thread_clone = std::sync::Arc::clone(&atomic);
    let atomic_count = std::sync::Arc::strong_count(&atomic);
    println!("Atomic vec: {atomic:?}, references: {atomic_count}");

    // Pin<Box<T>> - pinned heap allocation
    let pinned = pin_box!(100);
    println!("Pinned value: {pinned}");

    // Vectors of boxed values
    let boxes = vec_box![1, 2, 3, 4, 5];
    let values: Vec<_> = boxes.iter().map(|b| **b).collect();
    println!("Vector of boxes: {values:?}");
}
