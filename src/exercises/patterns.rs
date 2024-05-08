#![allow(unused)]

pub fn start() {
    // Pattern = some combo of:
    //  - Literals
    //  - Destructured arrays/enums/structs/tuples
    //  - Variables
    //  - Wildcards
    //  - Placeholders

    // e.g. x, (a, 3), Some(Colour::Red)

    // irrefutable: match for any possible value passed
    // refutable: can fail to match

    // PLACES PATTERNS CAN BE USED
    // I. match Arms
    //  - Must be exhaustive (catchall as last arrow helps)
    let x = Some(1);
    match x {
        None => None,
        Some(i) => Some(i + 1),
    };

    // II. if let Expressions
    //  - More flexible than match
    let favourite_colour: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(colour) = favourite_colour {
        println!("Using your favourite colour, {colour}, as the background");
    } else if is_tuesday {
        println!("Tuesday is a green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background");
        } else {
            println!("Using orange as the background");
        }
    } else {
        println!("Using blue as the background");
    }

    // III. while let Conditional Loops
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // IV. for Loops (irrefutable only)
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} @ index {}", value, index);
    }

    // V. let Statements (irrefutable only)
    //  let PATTERN = EXPRESSION;
    //  e.g. destructure tuple
    let (x, y, z) = (1, 2, 3);

    // VI. Function Parameters (irrefutable only)
    //  (can also use these in closure param lists)
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }
    let point = (3, 5);
    print_coordinates(&point);

    // PATTERN SYNTAX
    // I. Matching Literals
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("something else"),
    }

    // II. Matching Named Variables
    let x = Some(5);
    let y = 10;
    // match starts a new scope, so variables that are parts
    // of the pattern in match arms will shadow those with
    // the same name outside of match.
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched; y = {y}"),
        _ => println!("Default case; x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {y}", x);

    // III. Multiple Patterns
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("whatever"),
    }

    // IV. Matching Ranges of Values with ..=
    //  - numeric or char vals only
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("not a lowercase ASCII letter"),
    }

    // V. Destructuring to Break Apart Values
    //  - can destruct structs/enums/tuples
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
    // shorthand for patterns that match struct fields
    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
    // destruct with literal vals to test some fields for
    // particular vals whilst creating variables to
    // destructure other fields.
    let p = Point { x: 0, y: 7 };
    match p {
        Point { x, y: 0 } => println!("on x axis @ {x}"),
        Point { x: 0, y } => println!("on y axis @ {y}"),
        Point { x, y } => {
            println!("({x}, {y})");
        }
    }

    // VI. Destructuring Enums
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColour(u8, u8, u8),
    }
    let msg = Message::ChangeColour(0, 160, 255);
    match msg {
        Message::Quit => {
            println!("Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in x dir {x} and y dir {y}");
        }
        Message::Write(text) => {
            println!("Text msg: {text}");
        }
        Message::ChangeColour(r, g, b) => {
            println!("Change colour to red {r}, green {g}, blue {b}");
        }
    }

    // VII. Destructuring Nested Structs and Enums
    enum Colour {
        Rgb(u8, u8, u8),
        Hsv(u8, u8, u8),
    }
    enum Message2 {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColour(Colour),
    }
    let msg = Message2::ChangeColour(Colour::Hsv(0, 160, 255));
    match msg {
        Message2::ChangeColour(Colour::Rgb(r, g, b)) => {
            println!("Change colour to red {r}, green {g}, and blue {b}");
        }
        Message2::ChangeColour(Colour::Hsv(h, s, v)) => {
            println!("Change colour to hue {h}, saturation {s}, and value {v}");
        }
        _ => (),
    }

    // VIII. Complex Struct/Tuple Destructuring
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    // IGNORING VALUES IN A PATTERN
    // I. Ignoring Entire Value With _
    //  - Useful for trait impls
    fn foo(_: i32, y: i32) {
        println!("foo only uses y param: {}", y);
    }
    foo(3, 4);

    // II. Ignoring Parts of Value With Nested _
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite existing custom val");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);
    let numbers = (2, 4, 8, 18, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}");
        }
    }

    // III. Ignoring Unused Variable by Preceding Name With _
    //  - useful for prototyping or just starting a project
    let _y = 3;

    // IV. Ignoring Remaining Parts of Value With ..
    struct Point3d {
        x: i32,
        y: i32,
        z: i32,
    }
    let origin = Point3d { x: 0, y: 0, z: 0 };
    match origin {
        Point3d { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("some numbers: {first}, {last}");
        }
    }

    // EXTRA CONDITIONALS WITH MATCH GUARDS
    // Match Guard: additional if condition after pattern in
    //  match arm that must also match for that arm to be
    //  chosen.
    //  - Downside: compiler doesn't try to check for
    //    exhaustiveness.
    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }
    // Can test against values of outer variables. Match guard
    // is not a pattern so it doesn't introduce new variables.
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched; n = {n}"),
        _ => println!("Default case; x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {y}", x);

    // @ BINDINGS
    // @ creates variable that holds value at same time value
    // is tested for pattern match.
    enum Message3 {
        Hello { id: i32 },
    }
    let msg = Message3::Hello { id: 5 };
    match msg {
        Message3::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message3::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message3::Hello { id } => println!("Found some other id: {}", id),
    }
}
