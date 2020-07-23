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
}


