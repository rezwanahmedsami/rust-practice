pub fn run(){
    // print string to console
    println!("Hello from the print.rs file");

    // print number to console with basic formatiing
    println!("Number: {}", 1);

    // multiple formatting
    println!("I love {}, and i gave him {} fruits", "tommy", 5);

    // positional arguments
    println!("I love {0}, and i gave him {1} fruits. he done {2} this.", "coco", 5, "eating");

    // named arguments
    println!(
        "{name} is a good boy, he likes to play {activity}",
        name = "sami",
        activity = "football"
    );

    // placholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // placholder for debug traits
    println!("{:?}", (12, true, "HEllo"));

    // Basic math
    println!("10 + 10 = {}", 10+10);
}