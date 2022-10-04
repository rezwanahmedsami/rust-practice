/* 
Premitive types --
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (Number of bits they take in memory)
Floats: f32, f64
Boolean: (bool)
Charachters: (char)
Tuples
Array
*/

pub fn run(){

    // default is "i32"
    let x = 1;

    // default is "f64"
    let y = 2.5;

    // Add explicit type
    let z: i64 = 234234234324354;

    // boolean
    let is_active: bool = true;

    // get boolean from expression
    let is_greater: bool = 10 < 5;

    // define char
    let a1 = 'a';
    // unicode
    let face = '\u{1F600}';

    // find max size
    println!("MAx i32: {}", std::i32::MAX);
    println!("MAx i64: {}", std::i64::MAX);
    println!("MAx i128: {}", std::i128::MAX);
    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}