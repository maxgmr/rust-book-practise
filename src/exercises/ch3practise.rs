#![allow(unused)]

pub fn start() {
    let c: f64 = 21.4;
    let f: f64 = -26.8;
    println!(
        "{:.2} F = {:.2} C, {:.2} C = {:.2} F",
        f,
        f_to_c(f),
        c,
        c_to_f(c)
    );
    for i in 0..10 {
        println!("{}", gen_nth_fibonacci(i));
    }
}

fn f_to_c(f: f64) -> f64 {
    (f - 32.0) * (5.0 / 9.0)
}

fn c_to_f(c: f64) -> f64 {
    (c * (9.0 / 5.0)) + 32.0
}

fn gen_nth_fibonacci(n: i128) -> i128 {
    if n == 0 {
        return 0;
    }
    let mut prev1: i128 = 0;
    let mut prev2: i128 = 1;
    for i in 0..n - 1 {
        let temp = prev2;
        prev2 = prev1 + prev2;
        prev1 = temp;
    }
    return prev2;
}
