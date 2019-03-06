use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}


impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    //str is immutable, fixed length and stack allocated
    //String is heap allocated, growable
    //let my_str = "hello";
    //let my_string = String::from(my_str); 

    let num = Number::from(32); 
    println!("My number struct is: {:?}", num);

    let int = 5;

    let num: Number = int.into();
    println!("My number is {:?}", num);
}
