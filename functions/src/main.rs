fn five() -> i32 {
    5
}

fn main() {

    another_function(5);

    print_labeled_measurement(5, 'h');

    let x = five();
    println!("x = {x}");

    let x1 = plus_one(5);
    println!("x1 = {x1}");

}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
