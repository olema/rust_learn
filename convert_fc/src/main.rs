use std::io;
// use std::cmp::Ordering;

fn main() {
    println!("Эта программа конвертирует цельсии в фаренгейты и наоборот.");
    println!("Задайте направление конвертации (0 - по умолчанию):");
    println!("  0 - фаренгейты в цельсии;");
    println!("  1 - цельсии в фаренгейты;");
    println!("  2 - выход.");

    loop {
        println!("0: Ф->Ц, 1: Ц->Ф, 2: выход?");

        let mut cf = String::new();
//        let arr = ["",

        io::stdin().read_line(&mut cf).expect("Filed to read line");

        let cf: u64 = match cf.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        if cf == 0 {
            // фарен -> цельс
            println!("Фаренгейты -> Цельсии");
            println!("Введите температуру в фаренгейтах:");
            let znach = input_znach();
            println!("Вы ввели: {znach}. Температура в цельсиях: {}", convert_fc(znach, cf));
        } else if cf == 1 {
            println!("Цельсии -> Фаренгейты");
            println!("Введите температуру в цельсиях:");
            let znach = input_znach();
            println!("Вы ввели: {znach}. Температура в фаренгейтах: {}", convert_fc(znach, cf));
        } else if cf == 2 {
            println!("Goodby!");
            break
        }
    }
}

fn input_znach() -> i64 {
    let mut znach = String::new();
    io::stdin().read_line(&mut znach).expect("Filed to read line");
    let znach: i64 = match znach.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    znach
}

fn convert_fc(x:i64, n: u64) -> i64 {
//    (Фаренгейт — 32) : 1,8 = Цельсий Пример: (50°F - 32) : 1,8 = 10°C
//    Цельсий х 1,8 + 32 = Фаренгейт Пример: 10°C x 1,8 + 32 = 50°F
    let mut res: i64 = 0;
    if n == 0 {
        res = (x - 32)/2;
    } else {
        res = (x * 2) + 32;
    }
    res
}

