#![allow(unused)]

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn start() {
    // Message Passing: thread/actor communication via sending
    //      each other messages containing data.
    //      - important for safe concurrency!

    // mpsc = multiple producer, single consumer
    let (tx, rx) = mpsc::channel();

    // spawned thread must own transmitter to be able to send
    // messages through the channel.
    thread::spawn(move || {
        let val = String::from("wass6p");
        tx.send(val).unwrap();
    });

    // recv() wait until value sent, return as Result<T,E>.
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
    // try_recv() doesn't wait. returns Result<T,E>; Ok if
    // available message, otherwise Err

    // create multiple producers by cloning the transmitter
    let (tx2, rx2) = mpsc::channel();
    let tx3 = tx2.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("wass6p"),
            String::from("it's"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx3.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx2 {
        println!("Got: {}", received);
    }
}
