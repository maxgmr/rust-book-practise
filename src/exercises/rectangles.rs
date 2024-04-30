#![allow(unused)]

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // methods
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_eat(&self, other_rect: &Rectangle) -> bool {
        (self.width > other_rect.width && self.height > other_rect.height)
            || (self.width > other_rect.height && self.height > other_rect.width)
    }

    // associated functions: don't have self as first parameter, don't need instance of type to work with
    // often used for constructors that return new struct instances
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

pub fn start() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Rectangle area is {} square pixels", rect1.area());

    if rect1.width() {
        println!("Rectangle width is nonzero: {}", rect1.width);
    }

    println!("Can rect1 eat rect2? {}", rect1.can_eat(&rect2));
    println!("Can rect1 eat rect3? {}", rect2.can_eat(&rect3));

    let sq = Rectangle::square(45);
    println!("Can sq eat rect2? {}", sq.can_eat(&rect2));
}
