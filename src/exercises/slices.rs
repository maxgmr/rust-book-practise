#![allow(unused)]

pub fn start() {
    let mut s = String::from("hello world");
    let hello = &s[..5]; // this slice contains a pointer to the byte at index 0 of s with a length value of 5
    let world = &s[6..]; // this slice contains a pointer to the byte at index 6 of s with a length value of 5
    let whole_thing = &s[..];
    first_word(&s);
    better_first_word(&s);

    // other slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3]; // stores ref to first element and a length
    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// Using &str instead of &String in the signature is better because then the same function can be used on both &String and &str values.
// Pass string slice directly, pass String as a slice or as a reference
fn better_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
