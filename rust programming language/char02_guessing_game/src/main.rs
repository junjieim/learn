use std::io;
use std::cmp::Ordering;
use rand::Rng;  // trait

fn main() {
    println!("Guess the number!");
    // println! -> macro

    let secret_number: u32 = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        // `let` -> 声明变量，变量默认不可变
        // `mut` -> 可变变量

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line.");
        // Result 类型，枚举，Rust特点

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };  // 转数字，并指定类型 u32

        // 模式匹配
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
