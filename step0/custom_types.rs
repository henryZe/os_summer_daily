#![allow(dead_code)]

use crate::List::*;

enum Status {
    Rich,
    Poor,
}
enum Work {
    Civilian,
    Soldier,
}

enum Number {
    Zero,
    One,
    Two,
}

enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    // 3.1 Structures
    #[derive(Debug)]
    struct Person<'a> {
        // The 'a defines a lifetime
        name: &'a str,
        age: u8,
    }
    // A unit struct
    // Unit structs, which are field-less, are useful for generics.
    struct Unit;
    // A tuple struct
    struct Pair(i32, f32);
    // A struct with two fields
    #[derive(Debug)]
    struct Point {
        x: f32,
        y: f32,
    }
    // Structs can be reused as fields of another struct
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Rectangle {
        // A rectangle can be specified by where the top left and bottom right
        // corners are in space.
        top_left: Point,
        bottom_right: Point,
    }
    // Create struct with field init shorthand
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    // Print debug struct
    println!("{:?}", peter);
    let point: Point = Point { x: 10.3, y: 0.4 };
    println!("point coordinates: ({}, {})", point.x, point.y);
    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);
    // Destructure the point using a `let` binding
    let Point { x: top_edge, y: left_edge } = point;
    let rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };
    // Instantiate a unit struct
    let _unit = Unit;
    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);
    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);
    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);

    fn rect_area(rect: Rectangle) -> f32 {
        (rect.bottom_right.x - rect.top_left.x) * (rect.top_left.y - rect.bottom_right.y)
    }
    println!("rectangle {:?}", rectangle);
    println!("rect_area {}", rect_area(rectangle));

    fn square(arg0: Point, arg1: f32) -> Rectangle {
        let tl: Point = Point { x: arg0.x, y: arg1 };
        let br: Point = Point { x: arg1, y: arg0.y };
        Rectangle { top_left: tl, bottom_right: br }
    }
    println!("rectangle {:?}", square(point, 100.0));

    // 3.2 Enums
    enum WebEvent {
        // An `enum` may either be `unit-like`,
        PageLoad,
        PageUnload,
        // like tuple structs,
        KeyPress(char),
        Paste(String),
        // or c-like structures.
        Click { x: i64, y: i64 },
    }
    // A function which takes a `WebEvent` enum as an argument and
    // returns nothing.
    fn inspect(event: WebEvent) {
        match event {
            WebEvent::PageLoad => println!("page loaded"),
            WebEvent::PageUnload => println!("page unloaded"),
            // Destructure `c` from inside the `enum`.
            WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
            WebEvent::Paste(s) => println!("pasted \"{}\".", s),
            // Destructure `Click` into `x` and `y`.
            WebEvent::Click { x, y } => {
                println!("clicked at x={}, y={}.", x, y);
            },
        }
    }

    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    enum VeryVerboseEnumOfThingsToDoWithNumbers {
        Add,
        Subtract,
    }
    impl VeryVerboseEnumOfThingsToDoWithNumbers {
        fn run(&self, x: i32, y: i32) -> i32 {
            match self {
                Self::Add => x + y,
                Self::Subtract => x - y,
            }
        }
    }
    // Creates a type alias
    type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

    // We can refer to each variant via its alias, not its long and inconvenient
    // name.
    let enum_add = Operations::Add;
    println!("{}", enum_add.run(12, 1));
    println!("{}", Operations::Subtract.run(12, 1));

    // 3.2.1 use
    // Explicitly `use` each name so they are available without
    // manual scoping.
    use crate::Status::{Poor, Rich};
    // Automatically `use` each name inside `Work`.
    use crate::Work::*;
    // Equivalent to `Status::Poor`.
    let status = Poor;
    // Equivalent to `Work::Civilian`.
    let work = Civilian;
    match status {
        // Note the lack of scoping because of the explicit `use` above.
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }
    match work {
        // Note again the lack of scoping.
        Civilian => println!("Civilians work!"),
        Soldier  => println!("Soldiers fight!"),
    }

    // 3.2.2 C-like
    // `enums` can be cast as integers.
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);

    // 3.2.3 linked-list
    enum List {
        // Cons: Tuple struct that wraps an element and a pointer to the next node
        Cons(u32, Box<List>),
        // Nil: A node that signifies the end of the linked list
        Nil,
    }
    // Methods can be attached to an enum
    impl List {
        // Create an empty list
        fn new() -> List {
            // `Nil` has type `List`
            Nil
        }
        // Consume a list, and return the same list with a new element at its front
        fn prepend(self, elem: u32) -> List {
            // `Cons` also has type List
            Cons(elem, Box::new(self))
        }
        fn len(&self) -> u32 {
            match *self {
                Cons(_, ref tail) => 1 + tail.len(),
                // Base Case: An empty list has zero length
                Nil => 0
            }
        }

}



