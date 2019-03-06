#![allow(unreachable_code)]

fn main() {
    let mut count = 0u32;
    
    println!("Initiating loop!");
    loop {
        count += 1;
        if count == 3{
            println!("three!");
            //skips the next iteration
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("ok, that's enough");
            //exit
            break;
        }
    }

    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // This would break only the inner loop
            //break;

            // This breaks the outer loop
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}
