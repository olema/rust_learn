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

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup0 = (10, 20, 30, 40);
    let (x1, y1, z1) = tup;
    println!("The value of y1 is: {y1}");
    // access to tuple elements with index
    let ten = tup0.0;
    let twenty = tup0.1;
    println!("{ten} {twenty}");

    // array
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a1 = [3; 5]; // same: let a1 = [3, 3, 3, 3, 3];
    let apr = months[3];
    let one = a[0];
    let three = a1[4];
    println!("{apr}, {one}, {three}");


}
