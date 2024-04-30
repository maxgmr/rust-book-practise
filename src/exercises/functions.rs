#![allow(unused)]

pub fn start() {
    println!("start");
    another_function();
    param_example(123);
    statements_and_expressions();
    let x = 5;
    println!("{x} plus one equals {}", inc(x));
}

fn another_function() {
    println!("another_function");
}

fn param_example(x: i32) {
    println!("the param is x. the argument is {x}.");
}

// This whole function definition is a statement.
fn statements_and_expressions() {
    let y = 6; // This is a statement.

    // z's block is an expression because it returns a value.
    let z = {
        let x = 3; // This is a statement.
        x + 1 // This is an expression.
    };
}

fn inc(int: i32) -> i32 {
    int + 1
}
