pub fn run(){
    let age = 35;


    // if/else with nested if/else
    if age >= 18 && age <= 20{
        println!("You are and adult, you are approved");
    }else if age >= 25{
        println!("you are too old");
    }else {
        println!("Yu are too small");
    }


    // Short hand if
    let is_of_age = if age >= 20 { true } else { false };
    println!("Is of age: {}", is_of_age);
}