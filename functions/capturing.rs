fn main() {
    use std::mem;

    let color = "green";

    let print = || println!("`color`: {}", color);

    print();
    print();

    let mut count = 0;

    // 'count' becomes part of the closure 'inc', so 'inc' must be mutable so it 
    // can  increment 'count'.
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    inc();
    inc();

    let movable = Box::new(3);

    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable); 
    };

    consume();
    //consume();
}
