use std::io::{self};
//use std::mem;

//valgrind --leak-check=full ./target/release/my_app

fn main() {
    println!("Калькулятор\n");
    println!("Формат: число операция число");
    println!("Операции: +, -, *, /");
    println!("Введите 'exit' для выхода");
    
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        if input == "exit" {
            println!("Выход из калькулятора");
            break;
        }
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        
        if parts.len() != 3 {
            println!("Ошибка: введите два числа и операцию (пример: 5 + 3)");
            continue;
        }
        
        let a: f64 = match parts[0].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Ошибка: '{}' не число", parts[0]);
                continue;
            }
        };
        
        let op = parts[1];
        
        let b: f64 = match parts[2].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Ошибка: '{}' не число", parts[2]);
                continue;
            }
        };
        
        let result = match op {
            "+" => a + b,
            "-" => a - b,
            "*" => a * b,
            "/" => {
                if b == 0.0 {
                    println!("Ошибка: деление на ноль!");
                    continue;
                }
                a / b
            }
            _ => {
                println!("Ошибка: неизвестная операция '{}'", op);
                continue;
            }
        };
        
        //let m = 1;
        //while m < 10{
        //    println!("inf ");
        //}
        //let ptr : *const i32 = std::ptr::null();
        //let link = &*ptr;
        //let arr = vec![1,2,3];
        //mem::forget(arr);


        println!("Результат: {} {} {} = {}", a, op, b, result);
    }
}