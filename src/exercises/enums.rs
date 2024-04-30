#![allow(unused)]

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColour(i32, i32, i32),
}

// can define methods on enums, too!
impl Message {
    fn call(&self) {
        // blah blah blah
    }
}

pub fn start() {
    // create enum instances
    // both of these values are of the same type: IpAddr
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    // Option enum: must convert Option<T> to T before performing T operations on it.
    // avoids assuming that something isn't null when it actually is
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
}

fn route(ip_kind: IpAddr) {}
