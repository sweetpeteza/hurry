use hurry_macros::shorthand;

struct MyType {
    value: i32,
}

#[shorthand]
impl MyType {
    pub fn new(value: i32) -> Self {
        MyType { value }
    }
}

#[test]
fn test_shorthand_macro() {
    let instance = my_type!(42);
    assert_eq!(instance.value, 42);
}
