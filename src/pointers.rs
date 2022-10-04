// Reference pointers - point to a resource memory
pub fn run(){

    // premitive array
    let arr1 = [1,2,3,4,5];
    let arr2 = arr1;

    println!("Array: {:?}", (arr1, arr2));


    // With  non-primitive, if you assign another variable to a pieace of data, the first variable will no longer hold the value. you'll need to use a reference (&) to point to the resource

    // Vector
    let vec1 = vec![1,2,3,4,5];
    let vec2 = &vec1;
    println!("VAlues: {:?}", (&vec1, vec2));
}