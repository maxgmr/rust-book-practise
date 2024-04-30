#![allow(unused)]

pub fn start() {
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends literal to String
    println!("{}", s);

    // move: assigning s1 means that the pointer, length, and capacity values of s1 are copied to s2. s1 is then unassigned for memory safety.
    let s1 = String::from("hello");
    let s2 = s1;

    // clone: create deep copy of s1. here, arbitrary code is being executed and the operation may be expensive.
    let s1 = String::from("hello");
    let s2 = s1.clone();

    // OWNERSHIP AND FUNCTIONS ANNOTATED EXAMPLE
    let s = String::from("hello"); // s comes into scope
    takes_ownership(s); // s's value moves into the function. hence, it is no longer valid here

    let x = 5; // x comes into scope
    makes_copy(x); // since i32 is Copy, x can still be used afterwards

    return_values_and_scope_example();
} // Here, x goes out of scope, then s. But, since s's value was moved, nothing special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // some_string goes out of scope, "drop" is called. Backing memory is freed.

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // some_integer goes out of scope. Nothing special happens.

fn return_values_and_scope_example() {
    let s1 = gives_ownership(); // gives_ownership moves its return value into s1
    let s2 = String::from("hello"); // s2 comes into scope
    let s3 = takes_and_gives_back(s2); // s2 moves into fn, which also moves its return value into s3

    // gives_ownership moves return value into the function that calls it
    fn gives_ownership() -> String {
        let some_string = String::from("yours"); // some_string comes into scope
        some_string // some_string is returned and moves out into the calling function
    };

    // takes a String and returns one
    fn takes_and_gives_back(a_string: String) -> String {
        // a_string comes into scope
        a_string // a_string is returned and moves out to the calling function
    }
}
