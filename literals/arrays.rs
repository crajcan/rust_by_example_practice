use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

/*
fn modify(slice: &mut[i32]) {
    slice[0] = 42;
}
*/

fn main() {
    // Fixed size-array
    let xs: [i32; 5] = [1,2,3,4,5];

    let ys: [i32; 500] = [0; 500];

    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    println!("array size: {}", xs.len());

    // arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&xs));
    println!("borrow the whole array as a slice"); 
    analyze_slice(&xs);
    
    // Slices can point to a section of an array 
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);
    
    // Out of bound indexing causes compilation error 
    //println!("{}", xs[5]);

/*
    let mut slice = &ys[1..4];
    println!("slice[0] = {}", slice[0]);
    println!("slice.len() = {}", slice.len());
   
    modify(slice);
*/
}
