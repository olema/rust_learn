fn main() {
    let number = 7;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

     if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let mut number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
    
    // loop
    loop {
        println!("{number}. again!");
        if number > 10 { break};
        number += 1;
    }

    // returning value from loop
    let result = loop {
        number += 1;
        if number > 15 {
            break number * 2; // break return value
        }
    };
    println!("result loop = {result}");

    // loop lables
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; // exit from outer loop
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // while 
    number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // for statement through the elements of collection
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
