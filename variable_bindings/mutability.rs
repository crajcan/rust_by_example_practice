fn main() {
    let _immutable_binding  = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    // Ok
    mutable_binding ++;

    println!("After mutation: {}", mutable_binding);

    //_immutable_binding += 1;
}

