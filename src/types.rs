/*
Primitive Types--
Integers: u8, u16, u32, u64, u128, i8, i16, i32, i64, i128 (number of bits they take in memory)
Floats: f32, f64
Boolean: (bool)
Characters: (Char)
Tuples
Arrays
*/

pub fn run(){
    // Default is "i32"
    let x = 1;

    // Default is "f64"
    let y = 2.5;

    //Add explicit type
    let z:i64 = 4512564;

    //Find max size 
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active: bool = true;

    //Get Boolean from expression
    let is_greater = 10 < 5;

    //Char
    let s1: char = 's';
    let face = '\u{1F600}';
    println!("{:?}", (x, y, z, is_active, is_greater, s1, face));

}