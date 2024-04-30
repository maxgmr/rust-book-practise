#![allow(unused)]

pub fn start() {
    // & = Reference: an address that can be followed to access the data stored at that address.
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    // &mut = Mutable Reference: can't have any other references to a value with a mutable reference.
    // this prevents data races.
    let mut s2 = String::from("hello");
    change(&mut s2);

    // can use braces to create a new scope, allowing multiple mutable references (but not simultaneous mutable references!)
    let mut s3 = String::from("hello");
    {
        let r1 = &mut s3;
    } // r1 goes out of scope here, so can make new reference without problems.
    let r2 = &mut s3;
}

// s is a reference to a String
fn calculate_length(s: &String) -> usize {
    // since calculate_length only has a reference, it cannot modify the original value.
    s.len()
} // here, s goes out of scope, but since it does not have ownership of what it refers to, it is not dropped.

fn change(some_string: &mut String) {
    some_string.push_str(", bro");
}
