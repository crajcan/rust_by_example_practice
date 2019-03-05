fn inner(long_lived_binding: i32) {
    // This binding only exists in this block 
    let short_lived_binding = 2;

    println!("inner short: {}", short_lived_binding);

    // This binding *shadows* the outer one
    let long_lived_binding = 5_f32;

    println!("inner long: {}", long_lived_binding);
}
 

fn main() {
    let long_lived_binding = 1;

    //This is a block, and has a smaller scope than the main function
    inner(long_lived_binding);
 
    //println!("outer short: {}:", short_lived_binding);
    println!("outer long: {}", long_lived_binding);

    let long_lived_binding = 'a';

    println!("outer long: {}", long_lived_binding); 
}
