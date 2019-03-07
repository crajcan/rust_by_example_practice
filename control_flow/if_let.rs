enum Foo {
    Bar,
    Baz,
    Qux(u32)
}

fn main() {
    let number = Some(7);
    
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    let letter: Option<i32> = None;

    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        // Destructure failed. Change to the failure case.
        println!("The letter was empty!");
    }

        // Provide an altered failing condition.
    let i_like_letters = false;

    let emoticon: Option<i32> = Some(42); 

    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    // Destructure failed. Evaluate an `else if` condition to see if the
    // alternate failure branch should be taken:
    } else if i_like_letters {
        println!("I like letters!");
    } else {
        // The condition evaluated false. This branch is the default:
        println!("The emoticon was empty!");
    }

        // Create example variables
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);
    
    // Variable a matches Foo::Bar
    if let Foo::Bar = a {
        println!("a is foobar");
    }

    if let Foo::Baz = b {
        println!("b is a foobaz");
    } 

    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }
}
