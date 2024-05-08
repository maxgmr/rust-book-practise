#![allow(unused)]

use std::thread;
use std::time::Duration;

pub fn start() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    // When the main thread completes, all spawned threads are
    // shut down, whether or not they've finished running.
    // To make sure everything finishes, use JoinHandle.
    // When join called on JoinHandle, waits for its thread
    // to finish.
    // Blocks the currently running thread.
    handle.join().unwrap();

    // Move Closures With Threads
    // Commonly used with thread::spawn closures because the
    // closure takes ownership of values it uses from the
    // environment.
    let v = vec![1, 2, 3];
    // Force the closure to take ownership
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();
}
