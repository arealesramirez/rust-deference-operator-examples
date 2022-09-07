fn main() {
    dereferencing_shared_references();

    dereferencing_mutable_references();

    dereferencing_smart_pointers();

    dereferencing_raw_pointers();
}

fn dereferencing_shared_references() {
    let my_number = 1;
    let other_number = *&my_number;

    assert_eq!(my_number, other_number);
}

fn dereferencing_mutable_references() {
    let text = &mut String::from("foo");
    *text = String::from("bar");

    assert_eq!(text, "bar");
}

fn dereferencing_smart_pointers() {
    dereferencing_strings();

    dereferencing_structs();
}

// Strings are considered smart pointers as they own memory and allow you to manipulate
// it as well as having other metadata such as "capacity"
fn dereferencing_strings() {
    let string = String::from("Hello");
    // *string will generate a string literal str without a known size at compilation time
    // Hence, if you attempt,
    // let other_string = *string;
    // the code have errors at compilation time.

    let other_str = &*string;
    let other_string = String::from(&*string);

    assert_eq!(string, other_str);
    assert_eq!(string, other_string);
}

use std::ops::Deref;

#[derive(Debug)]
struct MyStruct {
    value: i32,
    another_value: String,
}

impl Deref for MyStruct {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

fn dereferencing_structs() {
    let my_struct = MyStruct {
        value: 1,
        another_value: String::from("a"),
    };
    let test = *my_struct;

    assert_eq!(1, test);
}

fn dereferencing_raw_pointers() {
    let test = "Hello!";
    let raw = &test as *const &str;
    let another_test = unsafe { *raw };

    assert_eq!("Hello!", another_test);
}
