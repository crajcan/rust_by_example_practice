fn main() {

    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }


    let names1 = vec!["Bob", "Frank", "Ferris"];
    for name in names1.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    // value after move error
    //println!("names: {:?}", names);

    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
                   &mut "Ferris" => "Rustacean",
                   _ => "Non-Rustacean",
        }
    }
    println!("names: {:?}", names);
}
