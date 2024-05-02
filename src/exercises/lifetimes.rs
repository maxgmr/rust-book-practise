#![allow(unused)]

use std::fmt::Display;

// ELISION RULES
// I.   Compiler assigns a separate lifetime param to each param that's a reference.
// II.  If exactly one input lifetime param, that lifetime is assigned to all output lifetime params.
// III. If there are multiple input lifetime params, but one of them is &self/&mut self because it's a method,
//      the lifetime of self is assigned to all output lifetime params.

// lifetime annotations in struct def'ns
// meaning: an instance of ImportantExcerpt can't outlive the reference it holds in its part field
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// lifetime annotations in struct impls
// must be declared because they're part of the struct's type
impl<'a> ImportantExcerpt<'a> {
    // not req'd to annotate &self (rule I).
    fn level(&self) -> i32 {
        3
    }

    // not req'd to annotate.
    // 2 input lifetimes; &self & announcment get their own lifetimes (rule I).
    // then, since one of the input params is &self, the return type gets the lifetime of &self (rule III).
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention: {}", announcement);
        self.part
    }
}

pub fn start() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        // putting the following line outside the braces wouldn't work, because it leaves the smaller scope of the two args
        println!("The longest string is {}", result);
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // static lifetimes: affected reference *can* live for the entire duration of the program.
    // all string literals have this lifetime. the text is stored in the program's binary, so it's always available.
    // don't use static lifetimes to fix dangling reference attempts or available lifetime mismatches!
    let s: &'static str = "I have a static lifetime.";
}

// problem: don't know whether returned ref will ref to x or y
// problem: don't know concrete lifetimes of passed-in refs
// solution: add generic lifetime params that define the relationship btwn the refs
// annotations tell Rust how generic lifetime params of multiple refs relate to each other

// example: for some lifetime 'a, the fn takes 2 string slices, both of which live at least as long as lifetime 'a.
// example: the string slice returned by fn will live at least as long as lifetime 'a.
// summary: lifetime of returned ref = the smaller of the lifetimes of the values ref'd to by the fn args
// this tells the borrow checker to reject any values that don't meet these conditions
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// combining generic type params, trait bounds, and lifetimes:
// returns longest of two string slices
// extra parameter of type T; T can be any type that implements the Display trait.
// 'a and T go together because lifetimes are a type of generic.
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
