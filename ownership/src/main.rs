fn main() {
    let s = String::from("hello");

//    takes_ownership(s); // s moved to some_string in function

//    println!("{}", s); // error
    
    takes_ownership(s.clone());
    println!("{}", s);

    let x = 5;

    makes_copy(x);
    println!("{}", x);

// return tuple
    let s1 = String::from("hello");

    let (s2, len) = calc_length(s1);

    println!("The length of '{}' is {}", s2, len);

}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn calc_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
