fn main() {
    let elem = 5u8;

    let mut vec = Vec::new();  // un typed at first

    // the program fails to compile without the next line
    vec.push(elem); 

    println!("{:?}", vec);
}
