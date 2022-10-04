
// vectors are resizable arrays

pub fn run(){
    // defining vector variable 
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    // re-asssign value by index number
    numbers[2] = 30;

    // add on to vector
    numbers.push(6);
    numbers.push(7);

    // pop off last value
    numbers.pop();

    // {:?} debug traits
    println!("{:?}", numbers);

    // get single val
    println!("Single val: {}", numbers[0]);

    // Get array length
    println!("Array length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Numbers: {:?}", slice);


    // Loop through vector values
    for x in numbers.iter(){
        println!("Number: {}", x);
    }

    // loop and mutate values
    for x in numbers.iter_mut(){
        *x *= 2;
    }
    println!("Numbers: {:?}", numbers)
}