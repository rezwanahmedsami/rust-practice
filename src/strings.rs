// Primitive str = Immutable fixed-length string somewhere in memory
//  String = Growable, heap-allocated data structure - use when you need to modify or own string data.

pub fn run(){
    let mut hello = String::from("Hello my name is ");

    println!("Length: {}", hello.len());

    // to push char on string
    hello.push('S');

    // to push string on string
    hello.push_str("ami");

    // capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // check is empty string or not
    println!("Is Empty: {}", hello.is_empty());

    // check contains
    println!("Contains 'Sami': {}", hello.contains("Sami"));

    // replace
    println!("Replace 'Sami' to 'sami': {}", hello.replace("Sami", "sami."));

    // loop through string by whitespace
    for word in hello.split_whitespace(){
        println!("{}", word);
    }

    // create string by capacity
    let mut s = String::with_capacity(10);
    s.push('S');
    s.push('a');
    s.push('m');
    s.push('i');
    println!("create name by capacity: {}", s);

    // Assertation testing
    assert_eq!(4, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", hello);
}