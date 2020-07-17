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
    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));
    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    println!("Now {:?} will print!", (Deep(Structure(7)).0).0);
    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    // Pretty print
    println!("{:#?}", peter);
    println!("{:?}", peter.name);

    // 1.2.2 Display
    

}
