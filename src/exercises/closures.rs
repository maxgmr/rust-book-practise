#![allow(unused)]

use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColour {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColour>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColour>) -> ShirtColour {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColour {
        let mut num_red = 0;
        let mut num_blue = 0;

        for colour in &self.shirts {
            match colour {
                ShirtColour::Red => num_red += 1,
                ShirtColour::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColour::Red
        } else {
            ShirtColour::Blue
        }
    }
}

pub fn start() {
    let store = Inventory {
        shirts: vec![ShirtColour::Blue, ShirtColour::Red, ShirtColour::Blue],
    };

    let user_pref1 = Some(ShirtColour::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}!",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}!",
        user_pref2, giveaway2
    );

    // All of these produce the same behaviour when called!
    // fn add_one_v1 (x: u32) -> u32 {x + 1}
    // let add_one_v2 = |x: u32| -> u32 {x + 1};
    // let add_one_v3 = |x| {x + 1};
    // let add_one_v4 = |x| x + 1;

    // Immutable reference closure
    let list = vec![1, 2, 3];
    println!("Before defining immutable ref closure: {:?}", list);
    let only_borrows = || println!("From closure: {:?}", list);
    println!("Before calling immutable ref closure {:?}", list);
    only_borrows();
    println!("After calling immutable ref closure {:?}", list);

    // Mutable reference closure
    let mut list2 = vec![1, 2, 3];
    println!("Before defining mutable ref closure: {:?}", list2);
    let mut borrows_mutably = || list2.push(4);
    println!(
        "Can't use list2 between definition and call of borrows_mutably. No other borrows allowed."
    );
    borrows_mutably();
    println!("After calling mutable ref closure: {:?}", list2);

    // Can use 'move' keyword to force closure to take ownership of the values it uses in the environment
    // Mostly useful when passing a closure to a new thread to move the data so that it's owned by the new thread
    let list3 = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list3);
    // The new thread might finish before the rest of the main thread finishes, or the main thread might finish first.
    // If the main thread retained ownership of list3 but ended before the new thread did and dropped list3, the
    // immutable reference in the thread would be invalid. Hence, the compiler requires that list3 be moved into the
    // closure given to the new thread so the reference will be valid.
    thread::spawn(move || println!("From thread: {:?}", list3))
        .join()
        .unwrap();

    // Closures will automatically implement one, two, or all three of these Fn traits,
    // depending on how the closure body handles the values:
    // I. FnOnce:
    //    -Applies to closures that can be called once. All closures implement at least
    //     this trait because all closures can be called. A closure that moves captured
    //     values out of its body will only implement FnOnce because it can only be
    //     called once.
    // II. FnMut:
    //   -Applies to closures that don't move captured values out of their body but
    //    that might mutate captured values. These closures can be called more than once.
    // III. Fn:
    //   -Applies to closures that don't move captured values out of their body and
    //    don't mutate captured values, as well as closures that capture nothing from
    //    their environments. These closures can be called more than once without
    //    mutating their environment, which is important in cases such as calling a
    //    closure multiple times concurrently.
}
