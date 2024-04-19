fn main() {
    let s1 = String::from("hello");
    let mut s2 = String::from("hello");

    let len = calc_length(&s1); // reference to data in s1
    
    println!("The length of '{}' is {}", s1, len);

//    change1(&s1); // error. s1 is not mutable
    change2(&mut s2);
    println!("{}", s2);

    let r1 = &mut s2;
//    let r2 = &mut s2; // error, if you have a mutable reference to a value, you can have no other references to that value

    let mut s3 = String::from("hello s3");

    let r2 = &s3; // immutable borrow, no problem
    let r3 = &s3; // immutable borrow, no problem
//    let r4 = &mut s3; // BIG PROBLEM, cannot borrow as mutable

    println!("r1 = {}, r2 = {}", r2, r3);
//variables r1 and r2 will not be used after this point

    let r4 = &mut s3; // no problem. 
    println!("r4 = {}", r4);

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



