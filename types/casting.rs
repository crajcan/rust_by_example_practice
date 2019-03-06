fn main() {
    let decimal = 65.4321_f32;

    //let integer: u8 = decimal;
    // ^^^ error: can't implicitly cast

    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);
   
    //1000 
    println!("1000 as a u16 is: {}", 1000 as u16);

                                       //        512 256 128 64  32  16  8   4   2   1
    //going downward we truncate (232)   1000 == 1   1   1   1   1   0   1   0   0   0
    // (out of range warning)
    println!("1000 as a u8 is : {}", 1000 as u8);

    //-1 + 256 (standard conversion) 
    println!("  -1 as a u8 is : {}", (-1i8) as u8);

    println!("1000 mod 256 is {}", 1000 % 256);

    // Unless it already fits, of course.
    println!(" 128 as a i16 is: {}", 128 as i16);
    // 128 as u8 -> 128, whose two's complement in eight bits is:
    println!(" 128 as a i8 is : {}", 128 as i8);

    // repeating the example above
    // 1000 as u8 -> 232
    println!("1000 as a u8 is : {}", 1000 as u8);

    // and the two's complement of 232 is -24 (interpret the above truncated ^^^ )
    println!(" 232 as a i8 is : {}", 232 as i8);
}
