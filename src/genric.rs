// When we want to create the function of multiple forms, i.e., the parameters of the function can accept the multiple types of data. This can be achieved through generics. Generics are also known as 'parametric polymorphism' where poly is multiple, and morph is form.

// enum Option<T>  
// {  
//     Some(T),  
//     None,  
// }

// struct Point<T>{
//     x: T,
//     y: T
// }

// impl <T> Point<T> {
//     fn x(&self) ->&T {
//         &self.x
//     }
// }

fn max_i32(a: i32, b: i32) -> i32 {
    if a > b {
        return a;
    }
    return b;
}

fn max_f64(a: f64, b: f64) -> f64 {
    if a > b {
        return a;
    }
    return b;
}

fn max_genric<T: PartialOrd> (a: T, b: T) -> T{
    if a > b {
        return a;
    }
    return b;
}

pub fn run(){

    // let x : Option<i32> = Some(10);  // 'T' is of type i32.  
    // let x : Option<bool> = Some(true);  // 'T' is of type bool.  
    // let x : Option<f64> = Some(10.5); // 'T' is of type f64.  
    // let x : Option<char> = Some('b'); // 'T' is of type char.   

    // let _p: Point<i32> = Point { x: 4, y: 5 };
    // println!("Generic struct is: {:?}", p);
    println!("Max i32 is: {}", max_i32(20, 10));
    println!("Max f64 is: {}", max_f64(20.34, 10.35));
    println!("Max genric for i32 is: {}", max_genric(50, 5));
    println!("Max genric for f64 is: {}", max_genric(50.3, 5.5));
}