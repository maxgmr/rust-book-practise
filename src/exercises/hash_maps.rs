#![allow(unused)]

use std::collections::HashMap;

pub fn start() {
    // Creating & adding to HashMap
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Accessing HashMap vals
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // Iterating over HashMap vals. Arbitrary order.
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Ownership
    // For types that implement the Copy trait, values are copied into HashMap.
    // For other types, HashMap takes ownership.
    // Can insert references- the values pointed to by the refs must be valid for at least as long as the HashMap is valid.

    // Updating HashMaps: overwriting a value
    scores.insert(String::from("Red"), 0);
    println!("{:#?}", scores);
    scores.insert(String::from("Red"), 12);
    println!("{:#?}", scores);

    // Updating HashMaps: add K,V only if K isn't present
    // entry takes wanted K as param, returns Entry enum that represents value that may or may not exist
    scores.entry(String::from("Orange")).or_insert(39);
    scores.entry(String::from("Blue")).or_insert(0);
    println!("{:#?}", scores);

    // Updating HashMaps: update value based on old value
    let text = "all human beings are born free and equal in dignity and rights they are endowed with reason and conscience and should act towards one another in a spirit of brotherhood";
    let mut word_count = HashMap::new();
    for word in text.split_whitespace() {
        // or_insert returns mutable reference to the value for the specified key
        let count = word_count.entry(word).or_insert(0);
        // to assign to that value, must dereference count
        *count += 1;
    }
    // mutable reference now out of scope, so all these changes are safe
    let text = "";
    println!("{:?}", word_count);
}
