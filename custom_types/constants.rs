//statics are changeable and have a static lifetime but they are unsafe to change? lets use consts for now 
static LANGUAGE: &str = "Rust";
//consts are unchangeable
const THRESHOLD: i32 = 10;


fn is_big(n: i32) -> bool {
    // Access constant in some function
    n > THRESHOLD
}

fn main () {
    let n = 16;
 
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });
}

