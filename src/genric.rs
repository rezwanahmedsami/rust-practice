// When we want to create the function of multiple forms, i.e., the parameters of the function can accept the multiple types of data. This can be achieved through generics. Generics are also known as 'parametric polymorphism' where poly is multiple, and morph is form.

// enum Option<T>  
// {  
//     Some(T),  
//     None,  
// }

struct Point<T>{
    x: T,
    y: T
}

impl <T> Point<T> {
    fn x(&self) ->&T {
        &self.x
    }
}

pub fn run(){

    // let x : Option<i32> = Some(10);  // 'T' is of type i32.  
    // let x : Option<bool> = Some(true);  // 'T' is of type bool.  
    // let x : Option<f64> = Some(10.5); // 'T' is of type f64.  
    // let x : Option<char> = Some('b'); // 'T' is of type char.   

    let _p: Point<i32> = Point { x: 4, y: 5 };
    // println!("Generic struct is: {:?}", p);

}