#![allow(unused)]

use std::ops::Deref;
use std::rc::Rc;

pub fn start() {
    // Smart Pointers
    // - can own the data they point to
    // - can have metadata
    // - can have extra capabilities/guarantees
    // - usually implemented as structs
    // - implement Deref and Drop traits
    // - Deref: allows instance to behave like a reference
    // - Drop: allows customisation of code that's run when instance leaves scope

    // Examples
    // Box<T>: Allocates values on heap
    // Rc<T>: Reference counting type that enables multiple ownership
    // Ref<T>, RefMut<T>, accessed through RefCell<T>: Enforces borrowing rules at runtime, not compile time

    // Box<T>, used in the following situations:
    // - Type of unknown size at compile time, and want to use val
    //   of that type in context that needs exact size
    // - Want to transfer ownership of large amt of data while
    //   ensuring data won't be copied
    // - Want to own a value and you care only that it's a type
    //   that implements a particular trait rather than being
    //   of a specific type

    // store i32 on heap, box is stored on stack
    let b = Box::new(5);
    println!("b = {}", b);

    // // cons list: from lisp. nested pairs akin to a linked list
    // // Boxes for recursive data types
    // enum List {
    //     Cons(i32, Box<List>),
    //     Nil,
    // }
    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    struct MyBox<T>(T);
    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }
    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    // Implementing Deref trait allows type to be dereferenced
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Deref coercions
    fn hello(name: &str) {
        println!("Hello, {}.", name);
    }
    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    // 3 Cases of Deref Coercion
    // I.   From &T to &U when T: Deref<Target=U>
    // II.  From &mut T to &mut U when T: DerefMut<Target=U>
    // III. From &mut T to &U when T: Deref<Target=U>

    // Implement Drop trait to specify what happens when a val
    // goes out of scope
    // e.g. Box<T> deallocates space on the heap the box points to
    struct CustomSmartPointer {
        data: String,
    }
    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data `{}`!", self.data);
        }
    }

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created");
    // Use std::mem::drop to force clean up value early
    std::mem::drop(c);

    // Rc<T>: reference counting (enables multiple ownership)
    //        single thread only
    #[derive(Debug)]
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    let a = Rc::new(List::Cons(
        5,
        Rc::new(List::Cons(10, Rc::new(List::Nil {}))),
    ));
    println!("ref count after creating a = {}", Rc::strong_count(&a));
    // Call Rc::clone to inc ref count and share ownership
    let b = List::Cons(3, Rc::clone(&a));
    println!("ref count after creating b = {}", Rc::strong_count(&a));
    {
        let c = List::Cons(4, Rc::clone(&a));
        println!("ref count after creating c = {}", Rc::strong_count(&a));
    }
    println!(
        "ref count after c goes out of scope = {}",
        Rc::strong_count(&a)
    );

    // Combining Rc<T> and RefCell<T> to have multiple owners
    // of mutable data
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};
    #[derive(Debug)]
    pub enum List2 {
        Cons(Rc<RefCell<i32>>, Rc<List2>),
        Nil,
    }
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(List2::Cons(Rc::clone(&value), Rc::new(List2::Nil)));
    let b = List2::Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = List2::Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    // Avoid reference cycles with weak references.
    // Call Rc::downgrade to get smart ptr type Weak<T>.
    // Increases weak_count by 1
    // To use value pointed to by Weak<T>, call 'upgrade',
    // which returns Option<Rc<T>>.
    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    // lack of infinite output shows that this isn't a reference cycle
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    // Visualise changes to strong_count and weak_count
    let leaf2 = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!(
        "leaf2 strong = {}, weak = {}",
        Rc::strong_count(&leaf2),
        Rc::weak_count(&leaf2),
    );
    {
        let branch2 = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf2)]),
        });
        *leaf2.parent.borrow_mut() = Rc::downgrade(&branch2);
        println!(
            "branch2 strong = {}, weak = {}",
            Rc::strong_count(&branch2),
            Rc::weak_count(&branch2),
        );
        println!(
            "leaf2 strong = {}, weak = {}",
            Rc::strong_count(&leaf2),
            Rc::weak_count(&leaf2),
        );
    }
    // as branch goes out of scope, weak count of 1 from leaf.parent
    // has no bearing on whether or not Node is dropped, so no
    // memory leaks!
    println!("leaf2 parent = {:?}", leaf2.parent.borrow().upgrade());
    println!(
        "leaf2 strong = {}, weak = {}",
        Rc::strong_count(&leaf2),
        Rc::weak_count(&leaf2),
    );
}

// RefCell<T>
// Allows data mutation when there are immutable refs to data
// Use unsafe code inside data structure, wrap in safe API,
// keep outer type immutable
// Reference/Box<T> borrowing rules enforced at compile time
// RefCell<T> borrowing rules enforced at runtime
// Breaking those rules with RefCell<T> causes panic
// Single thread only

// Interior Mutability Pattern
// Can't borrow immutable val mutably.
// However, can be useful to appear immutable to other code
// but mutate oneself in one's own methods

// Library code
pub trait Messenger {
    fn send(&self, msg: &str);
}
pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}
impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: Quota exceeded");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Warning: >90% of quota used");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Caution: >75% of quota useds");
        }
    }
}

// To test LimitTracker.set_value(), need a mock object that
// keeps track of the messages it's told to send
#[cfg(test)]
mod tests {
    use super::{Messenger, *};
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }
    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }
    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        mock_messenger.send("test message");

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
