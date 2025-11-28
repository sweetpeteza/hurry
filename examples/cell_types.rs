// Example demonstrating Cell and RefCell for interior mutability
use hurry::*;

fn main() {
    // Cell - for Copy types, provides interior mutability without runtime checks
    let cell_value = cell!(0);
    cell_value.set(10);
    cell_value.set(cell_value.get() + 5);
    println!("Cell value: {}", cell_value.get());

    // RefCell - for non-Copy types, provides interior mutability with runtime borrow checking
    let refcell_vec = refcell!(vec![1, 2, 3]);

    // Borrow mutably to modify
    refcell_vec.borrow_mut().push(4);
    refcell_vec.borrow_mut().push(5);

    // Borrow immutably to read
    println!("RefCell vec: {:?}", refcell_vec.borrow());

    // Demonstrate Cow (Clone-on-Write)
    let borrowed_str = cow_borrowed!("hello");
    println!("Borrowed Cow: {}", borrowed_str);

    let owned_string: std::borrow::Cow<'_, str> = cow_owned!(String::from("world"));
    println!("Owned Cow: {}", owned_string);
}
