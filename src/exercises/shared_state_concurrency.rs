#![allow(unused)]

use std::sync::{Arc, Mutex};
use std::thread;

pub fn start() {
    let m = Mutex::new(5);
    {
        // Use lock() to access the data inside the mutex
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);

    // Arc<T> = atomic Rc<T> for multithreading
    // Performance hit compared to Rc<T>
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // mutex provides interior mutability
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
    // for simple numerical operations, std::sync::atomic has
    // types simpler than Mutex

    // need to avoid deadlocks when using Mutex!
}
