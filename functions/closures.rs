fn main() {
    let i = 1;

    fn function (i: i32) -> i32 { i + 1 }
    println!("function: {}", function(i));

    let closure_annotated = |i: i32| -> i32 { i+ 1 };
    println!("closure_annotated: {}", closure_annotated(i));

    let closure_inferred = |i| i + 1;
    println!("closure_inferred: {}", closure_inferred(i));

    let one = || 1;
    println!("closure, returning one: {}", one());
}
