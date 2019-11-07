use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    //println!("The secret number is {}!", secret_number);

    loop {
        println!("Please Input your guess.");
        // 如果不是每个循环定义一个新的string,
        // readline好像不会覆写gues的值, 为啥?
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Fail to readline!");

        println!("You guessed : {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!!!");
                break;
            }
        }
    }
}
