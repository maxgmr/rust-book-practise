#![allow(unused)]

// Struct definition
// using String instead of &str means that each instance of this struct owns all of its data.
// this also means that all of this data will be valid for as long as this struct is valid.
// need lifetime specifiers in order for a struct to store references.
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuple struct definition
struct Colour(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-like struct definition; acts like ()
struct AlwaysEqual;

pub fn start() {
    // create instance of struct
    let mut user1 = User {
        active: true,
        username: String::from("awesomeguy123"),
        email: String::from("mistah-awesome@gmail.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("mistah-awesome@awesomecompany.com");

    // update syntax
    let mut user2 = User {
        email: String::from("that-otha-guy@gmail.com"),
        ..user1
    };
    // now, user1 can't be used. if new username and email fields were given, then user1 would still be valid because
    // as bool and u64, active and sign_in_count both implement the Copy trait.

    let black = Colour(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
