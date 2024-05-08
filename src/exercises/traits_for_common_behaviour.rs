#![allow(unused)]

use self::gui::Screen;
mod gui {
    pub trait Draw {
        fn draw(&self);
    }

    pub struct Screen {
        // trait object = Box<dyn Draw>>
        // vector of stand-ins for any type inside a Box that
        // implements the Draw trait.

        // Different from using a generic type param with trait
        // bounds. Generic type params can only be subbed with
        // one concrete type at a time. Trait objects allow
        // for multiple concrete types to fill in for the trait
        // at runtime.

        // generics/trait bounds better for homogenous
        // collections, trait objects needed for heterogenous
        // collections.
        pub components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }

    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }

    impl Draw for Button {
        fn draw(&self) {
            println!(
                "Drawing {}x{} \"{}\" button",
                self.width, self.height, self.label
            );
        }
    }

    pub struct SelectBox {
        pub width: u32,
        pub height: u32,
        pub options: Vec<String>,
    }

    impl Draw for SelectBox {
        fn draw(&self) {
            println!(
                "Drawing {}x{} select box; Options = {}",
                self.width,
                self.height,
                self.options.join(", ")
            );
        }
    }
}

pub fn start() {
    // note: trait bounds on generics perform static dispatch.
    //       trait objects perform dynamic dispatch at runtime
    //       to figure out which method to call- runtime cost!
    use gui::{Button, Screen, SelectBox};
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
