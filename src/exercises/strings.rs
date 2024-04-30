#![allow(unused)]

pub fn start() {
    // creating strings
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");
    let introduction = String::from("我叫马克斯");

    // updating strings
    // push_str takes string slice as parameter because it doesn't want to take ownership
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 is {s1}");
    println!("s2 is still {s2}");

    // push single character
    let mut s3 = String::from("Maxwel");
    s3.push('l');

    // concatenation with +
    let s4 = String::from("Hello, ");
    let s5 = String::from("world!");
    // statement takes ownership of s4, appends a copy of the contents of s5, then returns ownership of the result.
    let s6 = s4 + &s5;

    // format! formats stuff like println! but returns a String
    let s7 = String::from("veni");
    let s8 = String::from("vidi");
    let s9 = String::from("vici");
    // format! does not take ownership of any of its parameters
    let s10 = format!("{s7}, {s8}, {s9}");
    println!("{s10} = {s7} + {s8} + {s9}");

    // can't use indexing syntax on String
    // instead, can use slicing to get particular bytes
    // be careful using ranges, because trying to slice only a part of a character's bytes will crash the program.
    let hello = "Здравствуйте";
    let za_de = &hello[0..4];
    println!("{za_de}");

    // iterating over strings
    // iterate over individual Unicode scalar values with chars
    for c in "你好".chars() {
        println!("{c}");
    }
    // iterate over each raw byte with bytes. note that each char is 3 bytes in this case
    for b in "你好".bytes() {
        println!("{b}");
    }
}
