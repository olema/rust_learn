fn main() {
    let s1 = String::from("hello");

    let len = calc_length(&s1); // reference to data in s1
    
    println!("The length of '{}' is {}", s1, len);

}

fn calc_length(s: &String) -> usize {
    s.len()
}
