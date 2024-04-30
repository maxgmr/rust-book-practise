#![allow(unused)]

pub fn start() {
    let mut v1: Vec<i32> = Vec::new();
    let vec_from_macro = vec![1, 2, 3];
    v1.push(1);
    v1.push(2);

    let v2 = vec![1, 2, 3, 4, 5];
    // access w indexing syntax (panics when out of index, good when requiring accessed element to be in Vector)
    let third: &i32 = &v2[2];
    println!("v2 element 3 is {}", third);
    // access w get
    let third: Option<&i32> = v2.get(2);
    match third {
        Some(third) => println!("v2 element 3 is {}", third),
        None => println!("v2 has no third element"),
    }

    // iterating thru Vector
    // iterating over immutable references
    let v3 = vec![100, 32, 57];
    for i in &v3 {
        println!("{i}");
    }
    // iterating over mutable references
    let mut v4 = vec![100, 32, 57];
    for i in &mut v4 {
        *i += 50;
    }

    // using enums to store multiple types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
