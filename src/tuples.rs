// Tuples group together values of different types
// MAx 12 Elements

pub fn run(){
    let person: (&str, &str, i8) = ("Sami", "Bangladesh", 20);
    println!("Name is: {}, Location is: {}, Age is: {}", person.0, person.1, person.2);
}