fn main() {

    let x = 5;
    println!("The value of x is: {x}");
//    x = 6; //error: cannot assign twice to immutable variable
    let mut y = 5;
    println!("The value of y is: {y}");
    y = 6;
    println!("The value of y is: {y}");

    const THREE_HOURS_IN_SECONDS: u16 = 60 * 60 * 3;
    println!("const THREE_HOURS_IN_SECONDS: u16 is: {THREE_HOURS_IN_SECONDS}");

    // scalar types
    let int8:  i8 = 10;
    let uint8: u8 = 8;
    // i16, u16, i32, u32, i64, u64, i128, u128, isize, usize
    // number literals:
    // Decimal: 98_222, Hex: 0xff, Octal 0o77, Binary: 0b1111_0000, Byte (u8 only): b'A'

    println!("The value of int8: i8 is: {int8}");
    println!("The value of uint8: u8 is: {uint8}");
}
