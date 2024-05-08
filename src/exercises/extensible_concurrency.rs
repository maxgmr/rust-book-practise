#![allow(unused)]

pub fn start() {
    // Send: the ownership of values implementing this trait
    //  can be transferred between threads
    //  - Most types are Send. Rc<T> is not one of them.

    // Sync: it's safe for types that implement this trait to
    //  be referenced from multiple threads.
    //  - Type T is Sync iff &T is Send.
    //  - Primitive types and types entirely composed of Sync
    //    are Sync.
    //  - Rc<T> and RefCell<T> are not Sync.

    // Send & Sync are unsafe to implement manually.

    // Many concurrency techniques are implemented as crates.
}
