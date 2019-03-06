//type aliasing
type NanoSecond = u64;
type Inch = u64;

// Use an attribute to silence warning.
#[allow(non_camel_case_types)]
type u64_t = u64;

fn main () {
    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch  = 2 as u64_t;

    // aliases are not new types!!
    // nothing will stop me from doing this
    println!("{} nanoseconds + {} inches = {} unit?!?",
             nanoseconds,
             inches,
             nanoseconds + inches);
    // these are mostly used to reduce boilerplate
}
