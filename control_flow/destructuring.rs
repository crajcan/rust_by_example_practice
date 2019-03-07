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

    struct Foo { x: (u32, u32), y: u32 }
    let foo = Foo { x: (1,2), y: 3 };
    let Foo { x: (a, b), y } = foo;

    println!("a = {}, b = {}, y = {}", a, b, y);

    //ordering doesn't matter for destructuring with named fields,
    //this will maintain x and y from foo
    let Foo { y: i, x: j }  = foo;
    println!("i = {:?}, j = {:?}", i, j);

    let Foo { y, .. } = foo;
    println!("y = {}", y);

    // This will give an error: poatter does not mention field `x`
    //just like with matching!
    //let Foo { y } = foo;
}

