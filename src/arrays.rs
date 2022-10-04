
pub fn run(){
    let mut numbers: [i32; 5] = [1,2,3,4,5];

    // re-asssign value by index number
    numbers[2] = 30;

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
}