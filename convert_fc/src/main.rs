use std::io;
// use std::cmp::Ordering;

fn main() {
    println!("Эта программа конвертирует цельсии в фаренгейты и наоборот.");
    println!("Задайте направление конвертации (1 - по умолчанию):");
    println!("  1 - фаренгейты в цельсии;");
    println!("  2 - цельсии в фаренгейты;");
    println!("  3 - выход.");

    loop {
        println!("1/2/3?");

        let mut cf = String::new();
        let mut znach = String::new();

        io::stdin().read_line(&mut cf).expect("Filed to read line");

        let cf: u8 = match cf.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        if cf == 1 {
            println!("cf = {cf}");
        } else if cf == 2 {
            println!("cf = {cf}");
        } else if cf == 3 {
            println!("Goodby!");
            break
        }
    }
}

fn 
