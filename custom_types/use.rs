enum Status {
    Rich,
    Poor,
}

enum Work {
     Civilian,
     Soldier,
}

enum Number{
     foo,
     One,
     Two,
     bar,
}

enum Color {
    Red   = 0xff0000,
    Green = 0x00ff00,
    Blue  = 0x0000ff,
}

fn main() {
    use Status::{Poor, Rich};
    use Work::*;
 
    let status = Poor;
    let work = Civilian;

    match status {
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money...")
    }

    match work {
        Civilian => println!("Civilians work!"),
        Soldier  => println!("Soldiers fight!"),
    }

    println!("foo is {}",  Number::foo as i32);
    println!("one is {}",  Number::One as i32);
    println!("bar is {}",  Number::bar as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}
