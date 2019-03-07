fn main() {
    let pair = (1, 3);
    // TODO ^ Try different values for `pair`

    println!("Tell me about {:?}", pair);
    // Match can be used to destructure a tuple
    match pair {
        // Destructure the second
        (0, y) => println!("First is `0` and `y` is `{:?}`", y),
        (x, 0) => println!("`x` is `{:?}` and last is `0`", x),
        (x, y) => println!("`x` is `{:?}` and `y` is `{:?}`", x, y),
        // `_` means don't bind the value to a variable
    }
}

