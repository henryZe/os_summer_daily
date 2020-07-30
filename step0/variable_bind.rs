fn main() {

    // 4 Variable Bindings
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    let _unused_variable = 3u32;

    // 4.1. Mutability
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);
    mutable_binding += 1;
    println!("After mutation: {}", mutable_binding);

    // Error!
    // _immutable_binding += 1;

    // 4.2. Scope and Shadowing
    let long_lived_binding = 1;

    // This is a block, and has a smaller scope than the main function
    {
        // This binding only exists in this block
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);

        // This binding *shadows* the outer one
        let long_lived_binding = 5_f32;

        println!("inner long: {}", long_lived_binding);
    }

    // Error! `short_lived_binding` doesn't exist in this scope
    // println!("outer short: {}", short_lived_binding);

    println!("outer long: {}", long_lived_binding);
    
    // This binding also *shadows* the previous binding
    let long_lived_binding = 'a';
    println!("outer long: {}", long_lived_binding);

    // 4.3. Declare first
    // Declare a variable binding
    let a_binding;
    {
        let x = 2;

        // Initialize the binding
        a_binding = x * x;
    }
    println!("a binding: {}", a_binding);

    let another_binding;
    // Error! Use of uninitialized binding
    // println!("another binding: {}", another_binding);
    another_binding = 1;
    println!("another binding: {}", another_binding);

    // 4.4. Freezing
    let mut _mutable_integer = 7i32;
    {
        // Shadowing by immutable `_mutable_integer`
        let _mutable_integer = _mutable_integer;

        // Error! `_mutable_integer` is frozen in this scope
        // _mutable_integer = 50;

        // `_mutable_integer` goes out of scope
    }

    // Ok! `_mutable_integer` is not frozen in this scope
    _mutable_integer = 3;
}
