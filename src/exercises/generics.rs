#![allow(unused)]

struct Point<T> {
    x: T,
    y: T,
}

// only have to declare types after impl when generic...
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

//...not when concrete
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct LoosePoint<T, U> {
    x: T,
    y: U,
}

impl<T1, U1> LoosePoint<T1, U1> {
    fn mixup<T2, U2>(self, other: LoosePoint<T2, U2>) -> LoosePoint<T1, U2> {
        LoosePoint {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn start() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number in number_list is {}", result);

    let number_list2 = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list2);
    println!("The largest number in number_list2 is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char in char_list is {}", result);

    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
    let int_fl_point = LoosePoint { x: 5, y: 9.405 };

    let lp1 = LoosePoint { x: 5, y: 10.4 };
    let lp2 = LoosePoint { x: "Hello", y: 'c' };

    let lp3 = lp1.mixup(lp2);

    println!("lp3.x = {}, lp3.y = {}", lp3.x, lp3.y);
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
