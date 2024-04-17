fn main() {
    let s = String::from("hello");

//    takes_ownership(s); // s moved to some_string in function

//    println!("{}", s); // error
    
    takes_ownership(s.clone());
    println!("{}", s);

    let x = 5;

    makes_copy(x);
    println!("{}", x);

}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
