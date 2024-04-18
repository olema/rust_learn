fn main() {
    let s1 = String::from("hello");
    let mut s2 = String::from("hello");

    let len = calc_length(&s1); // reference to data in s1
    
    println!("The length of '{}' is {}", s1, len);

//    change1(&s1); // error. s1 is not mutable
    change2(&mut s2);
    println!("{}", s2);

    let r1 = &mut s2;
//    let r2 = &mut s2; // error, f you have a mutable reference to a value, you can have no other references to that value

}

fn calc_length(s: &String) -> usize {
    s.len()
}

//fn change1(s: &String) {
//    s.push_str(", world");
//}

fn change2(s: &mut String) {
    s.push_str(", world");
}



