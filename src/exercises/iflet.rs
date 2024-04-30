#![allow(unused)]

pub fn start() {
    // handle values that match one pattern whilst ignoring the rest
    let config_max = Some(3u8);

    // this...
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // ...is equivalent to this
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}
