#![allow(unused)]

pub fn start() {
    let mut x = 5;
    println!("x = {x}");
    x = 3;
    println!("x = {x}");

    // Shadowing
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("y in inner scope = {y}");
    }
    println!("y = {y}");

    // Arithmetic
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    println!("truncated = {truncated}");

    // char type
    let wo3 = '我';
    let shi4 = '是';
    let ma3 = '马';
    let ke4 = '克';
    let si1 = '斯';

    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("2nd value of tup = {y}");
    println!("3rd value of tup = {}", tup.2);

    // arrays: fixed size
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let arr_of_threes = [3; 5];
    println!("4th val of arr = {}", arr[3]);
}
