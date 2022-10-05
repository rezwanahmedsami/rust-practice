use std::fs::File;
use std::io::prelude::*;

pub fn run(){
    // let mut file = File::open("/Users/techcity/devwork/rust-work/practice/src/info.txt").expect("Can't open file");
    // let mut contents = String::new();

    // file.read_to_string(&mut contents)
    //     .expect("Can't read file");

    // println!("File contents: {}", contents);

    let mut file = File::create("/Users/techcity/devwork/rust-work/practice/src/created_info.txt").expect("Can't create file");
    file.write_all(b"Hello created file")
    .expect("Can't write to the file");
}   