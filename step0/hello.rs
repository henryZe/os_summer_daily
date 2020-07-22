use std::fmt::{self, Formatter, Display};

fn main() {
    // 1 hello world
    println!("hello world!");
    println!("I'm a Rustacean!");

    // 1.1 Comment
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100 ? x = {}", x);

    // 1.2 Formatted Print
    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");
    println!("{} of {:b} people know binary, the other half don't", 1, 2);
    println!("{number:>width$}", number=1, width=6);
    println!("{number:>0width$}", number=1, width=6);
    println!("My name is {0}, {1} {0}", "Bond", "James");
    let pi = 3.141592;
    println!("Pi is roughly {0:.1$}", pi, 3);

    // 1.2.1 Debug
    #[derive(Debug)]
    struct Structure(i32);
    #[derive(Debug)]
    struct Deep(Structure);
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");
    println!("Now {:?} will print!", Structure(3));
    println!("Now {:?} will print!", (Deep(Structure(7)).0).0);
    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    println!("{:#?}", peter);
    println!("{:?}", peter.name);

    sub_main();
}

fn sub_main() {
    // 1.2.2 Display
    struct Structure1(i32);
    impl fmt::Display for Structure1 {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    let case0 = Structure1(666);
    println!("case0: {}", case0);

    let minmax = MinMax(0, 14);
    println!("Compare structure:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    #[derive(Debug)]
    struct MinMax(i64, i64);
    impl fmt::Display for MinMax {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.0, self.1)
        }
    }

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
             small = small_range,
             big = big_range);

    #[derive(Debug)]
    struct Point2D {
        x: f64,
        y: f64,
    }
    impl fmt::Display for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "x: {}, y: {}", self.x, self.y)
        }
    }
    impl fmt::Binary for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "x: {}, y: {}", self.x, self.y)
        }
    }

    let point = Point2D { x: 3.3, y: 7.2 };
    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
    println!("What does Point2D look like in binary: {:b}?", point);

    #[derive(Debug)]
    struct Complex {
        real: f64,
        imag: f64,
    }
    impl fmt::Display for Complex {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} + {}i", self.real, self.imag)
        }
    }

    let complex = Complex { real: 3.3, imag: 7.2 };
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);

    // 1.2.2.1 Test case: List
    struct List(Vec<i32>);
    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // list, local variable
            let vec = &self.0;

            write!(f, "[")?;
            for (count, v) in vec.iter().enumerate() {
                if count != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}: {}", count, v)?;
            }
            write!(f, "]")
        }
    }

    let v = List(vec![1, 2, 3]);
    println!("{}", v);

    // 1.2.3 Formatting
    struct City {
        name: &'static str,
        // Latitude
        lat: f32,
        // Longitude
        lon: f32,
    }
    impl Display for City {
        // `f` is a buffer, and this method must write the formatted string into it
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            let lat_c = if self.lat >= 0.0 {'N'} else {'S'};
            let lon_c = if self.lon >= 0.0 {'E'} else {'W'};

            write!(f, "{}: {:.3}°{} {:.3}°{}",
                    self.name, self.lat.abs(), lat_c,
                    self.lon.abs(), lon_c)
        }
    }

    #[derive(Debug)]
    struct Color {
        red: u8,
        green: u8,
        blue: u8,
    }
    impl Display for Color {
        // `f` is a buffer, and this method must write the formatted string into it
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {

            write!(f, "RGB ({}, {}, {}) 0x{:02X?}{:02X?}{:02X?}",
                    self.red, self.green, self.blue,
                    self.red, self.green, self.blue)
        }
    }

    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", *city);
    }

    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        println!("{}", *color);    
    }
}
