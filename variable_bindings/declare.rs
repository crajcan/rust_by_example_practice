fn main() {
    let mut a_binding = 1;

    {
        let x = 2;
        println!("inner a_binding: {}", a_binding);
        a_binding = x * x
    }

    println!("outer a_binding: {}", a_binding);

    let another_binding;

    //println!("another binding: {}", another_binding);

    another_binding = 1;

    println!("another binding: {}", another_binding);
}
