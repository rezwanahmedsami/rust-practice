// Variables hold primitive data or reference to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run(){
    // define variable and print it
    // immutable variable
    let name = "sami";
    println!("My name is {}", name);

    // mutable variable and reassinging
    let mut age = 10;
    println!("My age is {}", age);
    // reassinging
    age = 20;
    println!("My age is {}", age);

    // constant variable
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple variables
    let (my_name, my_age) = ("sami", 20);
    println!("MY name is {} and i am {} years old", my_name, my_age);
}